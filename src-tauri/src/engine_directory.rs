use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use tar::Archive;

use crate::utils::get_app_data_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    name: String,
    description: String,
    website: String,
    license: String,
    version: String,
    updated_at: String,
    binaries: Vec<Binary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
    os: String,
    architecture: String,
    zip: String,
    binary_filename: String,
}

pub fn install(engine: Engine) -> PathBuf {
    let engines_path = get_app_data_dir().join("engines");

    assert!(
        engines_path.exists() || std::fs::create_dir(&engines_path).is_ok(),
        "Error creating engine directory at {:?}",
        engines_path.to_str().unwrap()
    );

    println!(
        "Downloading engine {} to {}",
        engine.name,
        engines_path.to_str().unwrap()
    );

    let os = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    };

    let architecture = "default";

    let binary = engine
        .binaries
        .iter()
        .find(|binary| binary.os == os && binary.architecture == architecture)
        .ok_or("No binary found for this system")
        .unwrap();

    let filename = Path::new(&binary.zip)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let zip_path = engines_path.join(filename);

    let mut resp = reqwest::blocking::get(&binary.zip).unwrap();
    let mut file = File::create(&zip_path).unwrap();
    io::copy(&mut resp, &mut file).unwrap();

    if binary.zip.ends_with(".zip") {
        let mut archive =
            zip::ZipArchive::new(File::open(&zip_path).unwrap()).expect("Error opening zip file");
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = engines_path.join(file.name());

            if (*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = Path::new(&outpath).parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    } else if binary.zip.ends_with(".tar") {
        let file = File::open(&zip_path).unwrap();
        let mut archive = Archive::new(file);

        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let outpath = engines_path.join(file.path().unwrap());

            if (*outpath.to_str().unwrap()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = Path::new(&outpath).parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    let path_to_binary = engines_path.join(&binary.binary_filename);

    println!("path_to_binary: {}", path_to_binary.to_str().unwrap());

    make_engine_executable(&path_to_binary);

    path_to_binary
}

#[cfg(target_family = "unix")]
fn make_engine_executable(path_to_binary: &PathBuf) {
    let mut perms = fs::metadata(&path_to_binary).unwrap().permissions();
    std::os::unix::prelude::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(&path_to_binary, perms).unwrap();
}

#[cfg(target_family = "windows")]
fn make_engine_executable(_path_to_binary: &PathBuf) {}
