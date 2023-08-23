#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use engine_directory::Engine;
use reqwest::Url;
use serde_json::{json, Value};
use std::{borrow::Cow, thread};
use sysinfo::{CpuExt, System, SystemExt};
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
fn add_engine(engine_id: &str, binary_location: &str) {
    db::add_engine(engine_id, binary_location);
}

#[tauri::command]
fn delete_engine(engine_id: &str) {
    db::delete_engine(engine_id);
}

#[tauri::command]
fn open_path(path: String) {
    utils::open_path(path);
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

// #[derive(serde::Deserialize, Debug)]
// struct AccessTokenResponse {
//     token_type: String,
//     access_token: String,
//     expires_in: i32,
// }

#[tauri::command]
async fn start_oauth_server() {
    let (code_challenge, code_verify) = oauth2::PkceCodeChallenge::new_random_sha256();

    let port = tauri_plugin_oauth::start_with_config(
        OauthConfig {
            ports: None,
            response: Some(Cow::Borrowed(include_str!("../public/oauth_response.html"))),
        },
        move |url| {
            println!("returning_from_lichess: {}", url);

            let url = Url::parse(&url).unwrap();
            println!("url: {:?}", url);

            let code = url.query_pairs().find(|(key, _)| key == "code").unwrap().1;

            println!("code_verify: {}", code_verify.secret());

            let lichess_host = db::get_setting("lichess_host").unwrap();

            let body = reqwest::blocking::Client::new()
                .post(format!("{}/api/oauth", lichess_host))
                .form(&[
                    ("grant_type", "authorization_code"),
                    ("client_id", "github.com/fitztrev/lichess-tauri"),
                    ("code", code.to_string().as_str()),
                    (
                        "redirect_uri",
                        format!("http://localhost:{}/", url.port().unwrap()).as_str(),
                    ),
                    ("code_verifier", code_verify.secret()),
                ])
                .send()
                .unwrap()
                .json::<serde_json::Value>()
                .unwrap();

            println!("body: {:?}", body);
        },
    )
    .unwrap();

    println!("Local server started on port: {}", port);
    let redirect_url = format!("http://localhost:{}/", port);

    let lichess_host = db::get_setting("lichess_host").unwrap();
    let url = format!(
        "{}/oauth?response_type=code&client_id={}&redirect_uri={}&code_challenge_method=S256&code_challenge={}&scope=engine:read%20engine:write",
        lichess_host,
        "github.com/fitztrev/lichess-tauri",
        redirect_url,
        code_challenge.as_str()
    );

    open_path(url);
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
            open_path
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
