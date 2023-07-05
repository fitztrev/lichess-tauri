#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use engine_directory::Engine;
use serde_json::{json, Value};
use std::{borrow::Cow, sync::Arc, thread};
use sysinfo::{CpuExt, System, SystemExt};
use tauri::Window;
use tauri_plugin_oauth::OauthConfig;

use crate::db::establish_connection;

mod engine_directory;
mod lichess;

pub mod db;
pub mod schema;
pub mod utils;

#[tauri::command]
fn get_all_settings() -> Value {
    let settings = db::get_all_settings();

    let mut json = json!({});
    for setting in settings {
        json[setting.key] = json!(setting.value);
    }

    json
}

#[tauri::command]
fn update_setting(key: &str, value: &str) {
    db::update_setting(key, value);
}

#[tauri::command]
fn delete_setting(key: &str) {
    db::delete_setting(key);
}

#[tauri::command]
fn add_engine(engine_id: &str, binary_location: &str, uci_options: &str) {
    db::add_engine(engine_id, binary_location, uci_options);
}

#[tauri::command]
fn delete_engine(engine_id: &str) {
    db::delete_engine(engine_id);
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

    let port = tauri_plugin_oauth::start_with_config(
        OauthConfig {
            ports: None,
            response: Some(Cow::Borrowed(include_str!("../public/oauth_response.html"))),
        },
        move |url| {
            window_arc2.emit("returning_from_lichess", url).unwrap();
        },
    )
    .unwrap();

    println!("Local server started on port: {}", port);
    window_arc.emit("server_started", port).unwrap();
}

#[tauri::command]
fn download_engine_to_folder(engine: Engine) -> String {
    engine_directory::install(engine)
        .into_os_string()
        .into_string()
        .unwrap()
}

#[tauri::command]
fn get_app_data_dir() -> String {
    utils::get_app_data_dir()
        .into_os_string()
        .into_string()
        .unwrap()
}

fn main() {
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    let mut connection = establish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_engine,
            delete_engine,
            delete_setting,
            download_engine_to_folder,
            get_all_settings,
            get_app_data_dir,
            get_sysinfo,
            start_oauth_server,
            update_setting,
        ])
        .setup(|app| {
            let app_handle = app.handle();

            thread::spawn(move || match lichess::work(&app_handle) {
                Ok(_) => println!("Success"),
                Err(e) => println!("Error: {}", e),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
