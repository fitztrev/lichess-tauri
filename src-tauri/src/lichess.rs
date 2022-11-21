use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use reqwest::{
    blocking::{Body, ClientBuilder},
    header::{self, HeaderMap},
};
use serde::{Deserialize, Serialize};
use tauri::Window;

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
    infinite: bool,
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

#[derive(Debug, Deserialize)]
pub struct EngineBinary {
    id: String,
    binary_location: String,
}

#[derive(Clone, Debug, Serialize)]
struct EventPayload {
    event: String,
    message: Option<String>,
    analysis_request: Option<AnalysisRequest>,
}

fn send_event_to_frontend(window: &Window, event: &str, payload: EventPayload) {
    println!("event: {} | {:?}", event, payload);
    window.emit(event, payload).unwrap();
}

pub fn work(
    engine_host: String,
    api_token: String,
    provider_secret: String,
    engine_binaries: Vec<EngineBinary>,
    window: Window,
) -> Result<(), Box<dyn Error>> {
    send_event_to_frontend(
        &window,
        "lichess::work",
        EventPayload {
            event: "start".to_string(),
            message: None,
            analysis_request: None,
        },
    );

    let mut default_headers = HeaderMap::new();
    default_headers.insert(header::AUTHORIZATION, api_token.try_into()?);
    let client = ClientBuilder::new()
        .default_headers(default_headers)
        .build()?;

    let provider_secret_borrowed = &provider_secret;
    let window_borrowed = &window;

    let mut backoff_duration_secs = 1;

    loop {
        // Step 1) Long poll for analysis requests
        // When a move is made on the Analysis board, it will be returned from this endpoint
        send_event_to_frontend(
            &window_borrowed,
            "lichess::work",
            EventPayload {
                event: "starting long poll".to_string(),
                message: None,
                analysis_request: None,
            },
        );
        let response = client
            .post(format!("{}/api/external-engine/work", engine_host))
            .json(&WorkRequest {
                provider_secret: provider_secret_borrowed.try_into()?,
            })
            .send()?;

        send_event_to_frontend(
            &window_borrowed,
            "lichess::work",
            EventPayload {
                event: "ending long poll".to_string(),
                message: Some(response.status().to_string()),
                analysis_request: None,
            },
        );

        if response.status() != 200 {
            send_event_to_frontend(
                &window_borrowed,
                "lichess::work",
                EventPayload {
                    event: "backing off".to_string(),
                    message: Some(format!("Backing off for {} seconds", backoff_duration_secs)),
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
            &window_borrowed,
            "lichess::work",
            EventPayload {
                event: "received analysis request".to_string(),
                message: None,
                analysis_request: Some(analysis_request.clone()),
            },
        );

        let engine_id = analysis_request.engine.id;
        let binary_filepath = engine_binaries
            .iter()
            .find(|engine_binary| engine_binary.id == engine_id)
            .ok_or("Engine not found")?
            .binary_location
            .clone();

        // Step 2) Send the FEN to the engine
        send_event_to_frontend(
            &window,
            "lichess::work",
            EventPayload {
                event: "starting engine".to_string(),
                message: Some(String::from(&binary_filepath)),
                analysis_request: None,
            },
        );

        let mut engine = Command::new(binary_filepath)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let engine_stdin = engine.stdin.as_mut().ok_or("Failed to get stdin")?;

        send_event_to_frontend(
            &window_borrowed,
            "lichess::work",
            EventPayload {
                event: "setting uci options".to_string(),
                message: None,
                analysis_request: None,
            },
        );
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
            "setoption name Variant value {}",
            analysis_request.work.variant
        )?;
        writeln!(
            engine_stdin,
            "position fen {} moves {}",
            analysis_request.work.initial_fen,
            analysis_request.work.moves.join(" ")
        )?;

        send_event_to_frontend(
            &window_borrowed,
            "lichess::work",
            EventPayload {
                event: "starting analysis".to_string(),
                message: None,
                analysis_request: None,
            },
        );
        if analysis_request.work.infinite {
            writeln!(engine_stdin, "go infinite")?;
        } else {
            writeln!(
                engine_stdin,
                "go depth {}\n",
                analysis_request.engine.default_depth
            )?;
        }

        engine_stdin.flush()?;

        let engine_stdout = engine.stdout.as_mut().ok_or("Failed to get stdout")?;

        let (tx, rx) = std::sync::mpsc::channel();
        let client = client.clone();

        let engine_host_for_thread = engine_host.to_string();
        let window_for_thread = window_borrowed.clone();

        std::thread::spawn(move || {
            // Step 3) Start a POST request stream to /api/external-engine/work/{id}
            let url = format!(
                "{}/api/external-engine/work/{}",
                engine_host_for_thread, analysis_request.id
            );
            send_event_to_frontend(
                &window_for_thread,
                "lichess::work",
                EventPayload {
                    event: "Starting thread to send analysis results".to_string(),
                    message: Some(String::from(&url)),
                    analysis_request: None,
                },
            );
            client
                .post(url)
                .body(Body::new(iter_read::IterRead::new(rx.into_iter().fuse())))
                .send()
        });

        for line in BufReader::new(engine_stdout).lines() {
            let mut line = line?;
            send_event_to_frontend(
                &window_borrowed,
                "lichess::work",
                EventPayload {
                    event: "engine result".to_string(),
                    message: Some(String::from(&line)),
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
                send_event_to_frontend(
                    &window_borrowed,
                    "lichess::work",
                    EventPayload {
                        event: "best move".to_string(),
                        message: Some(String::from(&line)),
                        analysis_request: None,
                    },
                );
                break;
            }
        }
    }
}
