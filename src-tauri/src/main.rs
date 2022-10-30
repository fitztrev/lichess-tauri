#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::{Command};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_engine(id: &str, fen: &str, moves: &str) {
    Command::new("/usr/bin/python3.10")
        .arg("/home/trevor/code/lichess-tauri/stockfish.py")
        .arg(id)
        .arg(fen)
        .arg(moves)
        .output()
        .expect("failed to execute process");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![run_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
