#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{process::{Command}, sync::Arc};
use serde_json::{json, Value};
use sysinfo::{System, SystemExt, CpuExt};

use tauri::{command, Window};

#[tauri::command]
fn run_engine(host: &str, token: &str, id: &str, binary: &str, fen: &str, moves: &str) {
    Command::new("python3")
        .args([
            "../stockfish.py",
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

#[tauri::command]
fn get_sysinfo() -> Value {
    let mut sys = System::new_all();
    sys.refresh_all();

    json!({
        "total_memory"   : sys.total_memory(),
        "used_memory"    : sys.used_memory(),
        "total_swap"     : sys.total_swap(),
        "used_swap"      : sys.used_swap(),
        "name"           : sys.name(),
        "kernel_version" : sys.kernel_version(),
        "os_version"     : sys.os_version(),
        "long_os_version": sys.long_os_version(),
        "host_name"      : sys.host_name(),
        "distribution_id": sys.distribution_id(),
        "cpus.len"       : sys.cpus().len(),
        "cpu.cpu_usage"  : sys.global_cpu_info().cpu_usage(),
        "cpu.brand"      : sys.global_cpu_info().brand(),
        "cpu.frequency"  : sys.global_cpu_info().frequency(),
        "cpu.vendor_id"  : sys.global_cpu_info().vendor_id(),
        "cpu.name"       : sys.global_cpu_info().name(),
    })
}

#[command]
async fn start_oauth_server(window: Window) {
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
            get_sysinfo,
            run_engine,
            start_oauth_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
