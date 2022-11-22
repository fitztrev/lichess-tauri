use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

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

pub fn download_to_folder(engine: Engine, folder: &str) -> String {
    println!("Downloading engine {:?} to {}", engine, folder);

    let binary = engine
        .binaries
        .iter()
        .find(|binary| binary.os == "linux" && binary.architecture == "default")
        .ok_or("No binary found for this system")
        .unwrap();

    let filename = Path::new(&binary.zip)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let mut resp = reqwest::blocking::get(&binary.zip).unwrap();
    let mut file = File::create(format!("{}/{}", folder, filename)).unwrap();
    io::copy(&mut resp, &mut file).unwrap();

    let mut archive =
        zip::ZipArchive::new(File::open(format!("{}/{}", folder, filename)).unwrap()).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = format!("{}/{}", folder, file.name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath);
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath,
                file.size()
            );
            if let Some(p) = Path::new(&outpath).parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    let filename_without_extension = Path::new(&filename).file_stem().unwrap().to_str().unwrap();
    format!(
        "{}/{}/{}",
        folder, filename_without_extension, binary.binary_filename
    )
}
