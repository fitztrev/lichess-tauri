use std::borrow::Cow;

use reqwest::Url;
use tauri::Window;
use tauri_plugin_oauth::OauthConfig;

use crate::{db, utils::open_path};

const OAUTH_CLIENT_ID: &str = "github.com/fitztrev/lichess-tauri";

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
struct AccessTokenResponse {
    token_type: String,
    access_token: String,
    expires_in: i32,
}

#[derive(serde::Deserialize, Debug)]
struct LichessAccount {
    username: String,
}

pub fn start_oauth_flow(window: Window) {
    let (code_challenge, code_verify) = oauth2::PkceCodeChallenge::new_random_sha256();

    let port = tauri_plugin_oauth::start_with_config(
        OauthConfig {
            ports: None,
            response: Some(Cow::Borrowed(include_str!("../public/oauth_response.html"))),
        },
        move |url| {
            let url = Url::parse(&url).unwrap();
            let code = url.query_pairs().find(|(key, _)| key == "code").unwrap().1;

            let lichess_host = db::get_setting("lichess_host").unwrap();

            let body = reqwest::blocking::Client::new()
                .post(format!("{}/api/token", lichess_host))
                .form(&[
                    ("grant_type", "authorization_code"),
                    ("client_id", OAUTH_CLIENT_ID),
                    ("code", code.to_string().as_str()),
                    (
                        "redirect_uri",
                        format!("http://localhost:{}/", url.port().unwrap()).as_str(),
                    ),
                    ("code_verifier", code_verify.secret()),
                ])
                .send()
                .unwrap()
                .json::<AccessTokenResponse>()
                .unwrap();

            db::update_setting("lichess_token", &body.access_token);

            let me = reqwest::blocking::Client::new()
                .get(format!("{}/api/account", lichess_host))
                .bearer_auth(&body.access_token)
                .send()
                .unwrap()
                .json::<LichessAccount>()
                .unwrap();

            db::update_setting("lichess_username", &me.username);
            window.emit("refresh_settings_from_database", ()).unwrap();
        },
    )
    .unwrap();

    let redirect_url = format!("http://localhost:{}/", port);
    println!("Local server started: {}", redirect_url);

    let lichess_host = db::get_setting("lichess_host").unwrap();
    let url = format!(
        "{}/oauth?response_type=code&client_id={}&redirect_uri={}&code_challenge_method=S256&code_challenge={}&scope=engine:read%20engine:write",
        lichess_host,
        OAUTH_CLIENT_ID,
        redirect_url,
        code_challenge.as_str()
    );

    open_path(url);
}

pub fn logout(window: Window) {
    let lichess_host = db::get_setting("lichess_host").unwrap();
    let token = db::get_setting("lichess_token").unwrap();

    reqwest::blocking::Client::new()
        .delete(format!("{}/api/token", lichess_host))
        .bearer_auth(&token)
        .send()
        .unwrap();

    db::delete_setting("lichess_token");
    db::delete_setting("lichess_username");

    window.emit("refresh_settings_from_database", ()).unwrap();
}
