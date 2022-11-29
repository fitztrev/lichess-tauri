#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use engine_directory::Engine;
use lichess::EngineBinary;
use rand::Rng;
use serde_json::{json, Value};
use std::sync::Arc;
use sysinfo::{CpuExt, System, SystemExt};
use tauri::Window;

use crate::db::{add_default_setting, add_engine, establish_connection};

mod engine_directory;
mod lichess;

pub mod db;
pub mod schema;

#[tauri::command]
fn check_for_work(
    engine_host: String,
    api_token: String,
    provider_secret: String,
    engine_binaries: Vec<EngineBinary>,
    window: Window,
) {
    std::thread::spawn(|| {
        match lichess::work(
            engine_host,
            api_token,
            provider_secret,
            engine_binaries,
            window,
        ) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    });
}

#[tauri::command]
fn stop_checking_for_work() {
    println!("called stop_checking_for_work");
}

#[tauri::command]
fn get_all_settings() -> Value {
    println!("called get_all_settings");

    let settings = db::get_all_settings();

    let mut json = json!({});
    for setting in settings {
        json[setting.key] = json!(setting.value);
    }

    json
}

#[tauri::command]
fn update_setting(key: String, value: String) {
    db::update_setting(key, value);
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
        "cpus_len"       : sys.cpus().len(),
        "cpu_cpu_usage"  : sys.global_cpu_info().cpu_usage(),
        "cpu_brand"      : sys.global_cpu_info().brand(),
        "cpu_frequency"  : sys.global_cpu_info().frequency(),
        "cpu_vendor_id"  : sys.global_cpu_info().vendor_id(),
        "cpu_name"       : sys.global_cpu_info().name(),
    })
}

#[tauri::command]
async fn start_oauth_server(window: Window) {
    let window_arc = Arc::new(window);
    let window_arc2 = window_arc.clone();
    let port = tauri_plugin_oauth::start(
        Some(
            "<html><head></head><body>You can close this tab and return to the app.</body></html>",
        ),
        move |url| {
            println!("Returning from oauth, url: {}", url);
            window_arc2.emit("returning_from_lichess", url).unwrap();
        },
    )
    .unwrap();

    println!("Local server started on port: {}", port);
    window_arc.emit("server_started", port).unwrap();
}

#[tauri::command]
fn download_engine_to_folder(engine: Engine, folder: &str) -> String {
    engine_directory::download_to_folder(engine, folder)
}

fn main() {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    let mut connection = establish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let provider_secret = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect::<String>();

    add_default_setting("lichess_host", "https://lichess.org");
    add_default_setting("engine_host", "https://engine.lichess.ovh");
    add_default_setting("provider_secret", provider_secret.as_str());

    let lichess_host = db::get_setting("lichess_host").unwrap();
    println!("lichess_host: {}", lichess_host);

    add_engine("eei_1234", "/path/to/stockfish");

    let binary_path = db::get_engine_binary_path("eei_1234").unwrap();
    println!("binary_path: {}", binary_path);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_engine_to_folder,
            get_all_settings,
            get_sysinfo,
            check_for_work,
            stop_checking_for_work,
            start_oauth_server,
            update_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
