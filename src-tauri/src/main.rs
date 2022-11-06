#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{process::{Command}, sync::Arc};
use tauri::{command, Window};

#[tauri::command]
fn run_engine(host: &str, token: &str, id: &str, binary: &str, fen: &str, moves: &str) {
    Command::new("/usr/bin/python3")
        .args([
            "/home/trevor/code/lichess-tauri/stockfish.py",
            "--host", host,
            "--token", token,
            "--work-id", id,
            "--binary-path", binary,
            "--fen", fen,
            "--moves", moves,
        ])
        .output()
        .expect("failed to execute process");
}

#[command]
async fn start_server(window: Window) {
  let window_arc = Arc::new(window);
  let window_arc2 = window_arc.clone();
  let port = tauri_plugin_oauth::start(None, move |url| {
    println!("Returning from oauth, url: {}", url);
    window_arc2.emit("returning_from_lichess", url).unwrap();
  }).unwrap();

  println!("Local server started on port: {}", port);
  window_arc.emit("server_started", port).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_engine,
            start_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
