use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    time::Duration,
};

use reqwest::{
    blocking::{Body, ClientBuilder},
    header::{self, HeaderMap},
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::db;

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnalysisRequest {
    id: String,
    work: Work,
    engine: Engine,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Work {
    session_id: String,
    threads: u32,
    hash: u32,
    movetime: u32,
    multi_pv: u32,
    variant: String,
    initial_fen: String,
    moves: Vec<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Engine {
    id: String,
    name: String,
    client_secret: String,
    user_id: String,
    max_threads: u32,
    max_hash: u32,
    default_depth: u32,
    variants: Vec<String>,
    provider_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Registration {
    name: String,
    max_threads: u32,
    max_hash: u32,
    default_depth: u32,
    variants: Vec<String>,
    provider_secret: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkRequest {
    provider_secret: String,
}

#[derive(Clone, Debug, Serialize)]
enum EventPayloadType {
    Status,
    Uci,
    Sleep,
}

#[derive(Clone, Debug, Serialize)]
struct EventPayload {
    event: EventPayloadType,
    message: String,
    analysis_request: Option<AnalysisRequest>,
}

#[derive(Clone, Debug, Serialize)]
enum StatusLevel {
    Info,
    Error,
}

#[derive(Clone, Debug, Serialize)]
struct StatusPayload {
    status: String,
    level: StatusLevel,
}

fn send_event_to_frontend(app_handle: &AppHandle, event: &str, payload: EventPayload) {
    println!("event: {} | {:?}", event, payload);
    app_handle.emit_all(event, payload).unwrap();
}

fn send_status_to_frontend(app_handle: &AppHandle, payload: StatusPayload) {
    app_handle
        .emit_all("lichess::send_status_to_frontend", payload)
        .unwrap();
}

pub fn work(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
    let mut backoff_duration_secs = 1;

    std::thread::sleep(std::time::Duration::from_secs(3));

    loop {
        let api_token = db::get_setting("lichess_token");
        match api_token {
            Some(_) => {}
            None => {
                send_status_to_frontend(
                    app_handle,
                    StatusPayload {
                        status: "Waiting for Lichess login".to_string(),
                        level: StatusLevel::Info,
                    },
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        }

        let provider_secret = db::get_setting("provider_secret");
        match provider_secret {
            Some(_) => {}
            None => {
                send_status_to_frontend(
                    app_handle,
                    StatusPayload {
                        status: "Missing provider secret".to_string(),
                        level: StatusLevel::Error,
                    },
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        }

        let engine_host = db::get_setting("engine_host");
        match engine_host {
            Some(_) => {}
            None => {
                send_status_to_frontend(
                    app_handle,
                    StatusPayload {
                        status: "Missing engine host setting".to_string(),
                        level: StatusLevel::Error,
                    },
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        }

        if db::get_engine_count() == 0 {
            send_status_to_frontend(
                app_handle,
                StatusPayload {
                    status: "Waiting for engine to be added".to_string(),
                    level: StatusLevel::Info,
                },
            );
            std::thread::sleep(std::time::Duration::from_secs(5));
            continue;
        }

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::AUTHORIZATION,
            api_token.expect("missing api_token").try_into()?,
        );
        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        // Step 1) Long poll for analysis requests
        // When a move is made on the Analysis board, it will be returned from this endpoint
        send_event_to_frontend(
            app_handle,
            "lichess::work",
            EventPayload {
                event: EventPayloadType::Status,
                message: "Waiting for moves".to_string(),
                analysis_request: None,
            },
        );
        let response = client
            .post(format!(
                "{}/api/external-engine/work",
                engine_host.clone().expect("missing engine_host")
            ))
            .json(&WorkRequest {
                provider_secret: provider_secret.expect("missing provider_secret"),
            })
            .send()?;

        if response.status() != 200 {
            send_event_to_frontend(
                app_handle,
                "lichess::work",
                EventPayload {
                    event: EventPayloadType::Sleep,
                    message: backoff_duration_secs.to_string(),
                    analysis_request: None,
                },
            );

            std::thread::sleep(std::time::Duration::from_secs(backoff_duration_secs));
            backoff_duration_secs = std::cmp::min(backoff_duration_secs * 2, 10);
            continue;
        }

        backoff_duration_secs = 1;

        let analysis_request = response.json::<AnalysisRequest>()?;

        send_event_to_frontend(
            app_handle,
            "lichess::work",
            EventPayload {
                event: EventPayloadType::Status,
                message: "Analyzing".to_string(),
                analysis_request: Some(analysis_request.clone()),
            },
        );

        let binary_filepath = db::get_engine_binary_path(&analysis_request.engine.id);
        match binary_filepath {
            Some(_) => {}
            None => {
                send_status_to_frontend(
                    app_handle,
                    StatusPayload {
                        status: "Missing binary filepath".to_string(),
                        level: StatusLevel::Error,
                    },
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        }

        // Step 2) Send the FEN to the engine
        let mut process = Command::new(binary_filepath.as_ref().expect("missing binary_filepath"));
        process.stdin(Stdio::piped()).stdout(Stdio::piped());

        // Hide the console window on Windows
        #[cfg(windows)]
        if cfg!(windows) {
            use std::os::windows::process::CommandExt;

            const CREATE_NO_WINDOW: u32 = 0x08000000;
            process.creation_flags(CREATE_NO_WINDOW);
        }

        let mut engine = process.spawn();

        match engine {
            Ok(_) => {}
            Err(e) => {
                send_status_to_frontend(
                    app_handle,
                    StatusPayload {
                        status: format!(
                            "Failed to start engine: {} for {}",
                            e,
                            binary_filepath.as_ref().unwrap()
                        ),
                        level: StatusLevel::Error,
                    },
                );
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }
        }

        let engine_stdin = engine.as_mut().unwrap().stdin.as_mut().unwrap();

        // Set UCI options
        writeln!(engine_stdin, "setoption name UCI_AnalyseMode value true")?;
        writeln!(engine_stdin, "setoption name UCI_Chess960 value true")?;
        writeln!(
            engine_stdin,
            "setoption name Threads value {}",
            analysis_request.work.threads
        )?;
        writeln!(
            engine_stdin,
            "setoption name Hash value {}",
            analysis_request.work.hash
        )?;
        writeln!(
            engine_stdin,
            "setoption name MultiPV value {}",
            analysis_request.work.multi_pv
        )?;
        writeln!(
            engine_stdin,
            "position fen {} moves {}",
            analysis_request.work.initial_fen,
            analysis_request.work.moves.join(" ")
        )?;

        writeln!(
            engine_stdin,
            "go movetime {}",
            analysis_request.work.movetime
        )?;

        engine_stdin.flush()?;

        let engine_stdout = engine.as_mut().unwrap().stdout.as_mut().unwrap();

        let (tx, rx) = std::sync::mpsc::channel();
        let client = client.clone();

        std::thread::spawn(move || {
            // Step 3) Start a POST request stream to /api/external-engine/work/{id}
            let url = format!(
                "{}/api/external-engine/work/{}",
                engine_host.expect("missing engine_host"),
                analysis_request.id
            );
            client
                .post(url)
                .body(Body::new(iter_read::IterRead::new(rx.into_iter().fuse())))
                .timeout(Duration::from_secs(600))
                .send()
        });

        for line in BufReader::new(engine_stdout).lines() {
            let mut line = line?;
            send_event_to_frontend(
                app_handle,
                "lichess::work",
                EventPayload {
                    event: EventPayloadType::Uci,
                    message: String::from(&line),
                    analysis_request: None,
                },
            );
            if line.starts_with("info") {
                line.push('\n');
                if tx.send(line).is_err() {
                    // sending thread stopped, meaning Lichess doesn't want any more analysis. wait for next request
                    break;
                }
            } else if line.starts_with("bestmove") {
                break;
            }
        }
    }
}
