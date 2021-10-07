use std::io::{Cursor};

use std::fs::File;
use std::path::Path;

pub fn init(download_path: &str) {
    // check if "downloads" folder exists
    let exists = Path::new(download_path).exists();
    if !exists {
        println!("[INIT] Download folder does not exist, creating");
        let _ignore = std::fs::create_dir(download_path);
    }
}

pub fn download_file(url: String, file_name: String, download_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url.as_str())?;
    let mut file = File::create(file_name.clone())?;
    let mut content = Cursor::new(res.bytes()?);
    std::io::copy(&mut content, &mut file)?;

    let mut path_to_file = download_path.to_owned();
    path_to_file.push_str("/"); // add seperator;
    path_to_file.push_str(file_name.as_str());
    std::fs::copy(file_name.clone(), path_to_file)?;

    std::fs::remove_file(file_name.clone())?;

    Ok(())
}