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

#[derive(Clone, Debug, Serialize)]
enum EventPayloadType {
    Status,
    Uci,
}

#[derive(Clone, Debug, Serialize)]
struct EventPayload {
    event: EventPayloadType,
    message: String,
    analysis_request: Option<AnalysisRequest>,
}

fn send_event_to_frontend(window: &Window, event: &str, payload: EventPayload) {
    println!("event: {} | {:?}", event, payload);
    window.emit(event, payload).unwrap();
}

pub fn work(window: Window) -> Result<(), Box<dyn Error>> {
    let api_token = db::get_setting("lichess_token").unwrap();
    let provider_secret = db::get_setting("provider_secret").unwrap();
    let engine_host = db::get_setting("engine_host").unwrap();

    let mut default_headers = HeaderMap::new();
    default_headers.insert(header::AUTHORIZATION, api_token.try_into()?);
    let client = ClientBuilder::new()
        .default_headers(default_headers)
        .build()?;

    let mut backoff_duration_secs = 1;

    loop {
        let engine_host = engine_host.clone();

        // Step 1) Long poll for analysis requests
        // When a move is made on the Analysis board, it will be returned from this endpoint
        send_event_to_frontend(
            &window,
            "lichess::work",
            EventPayload {
                event: EventPayloadType::Status,
                message: "Waiting for moves".to_string(),
                analysis_request: None,
            },
        );
        let response = client
            .post(format!("{}/api/external-engine/work", engine_host))
            .json(&WorkRequest {
                provider_secret: String::from(&provider_secret),
            })
            .send()?;

        send_event_to_frontend(
            &window,
            "lichess::work",
            EventPayload {
                event: EventPayloadType::Status,
                message: "No moves received".to_string(),
                analysis_request: None,
            },
        );

        if response.status() != 200 {
            send_event_to_frontend(
                &window,
                "lichess::work",
                EventPayload {
                    event: EventPayloadType::Status,
                    message: format!("Waiting for {} seconds", backoff_duration_secs),
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
            &window,
            "lichess::work",
            EventPayload {
                event: EventPayloadType::Status,
                message: "Analyzing".to_string(),
                analysis_request: Some(analysis_request.clone()),
            },
        );

        let binary_filepath = db::get_engine_binary_path(&analysis_request.engine.id).unwrap();

        // Step 2) Send the FEN to the engine
        println!("Starting engine: {}", binary_filepath);

        let mut engine = Command::new(binary_filepath)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let engine_stdin = engine.stdin.as_mut().ok_or("Failed to get stdin")?;

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

        std::thread::spawn(move || {
            // Step 3) Start a POST request stream to /api/external-engine/work/{id}
            let url = format!(
                "{}/api/external-engine/work/{}",
                engine_host, analysis_request.id
            );
            client
                .post(url)
                .body(Body::new(iter_read::IterRead::new(rx.into_iter().fuse())))
                .send()
        });

        for line in BufReader::new(engine_stdout).lines() {
            let mut line = line?;
            send_event_to_frontend(
                &window,
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
