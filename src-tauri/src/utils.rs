use std::path;

pub fn get_app_data_dir() -> path::PathBuf {
    let app_data_dir =
        path::Path::new(&tauri::api::path::local_data_dir().unwrap()).join("lichess-tauri");

    assert!(
        app_data_dir.exists() || std::fs::create_dir(&app_data_dir).is_ok(),
        "Error creating app data directory at {:?}",
        app_data_dir.to_str().unwrap()
    );

    app_data_dir
}
