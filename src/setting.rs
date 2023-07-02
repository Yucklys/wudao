use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Setting {}

impl Setting {
    /// Download data files.
    pub fn create<'a>(entries: Vec<DataEntry<'a>>) -> io::Result<()> {
        let data_dir = data_dir().unwrap();
        for p in entries {
            match p {
                // Create all directories
                DataEntry::Dir(p) => {
                    let path = data_dir.join(p);
                    std::fs::create_dir_all(path)?;
                }
                // Dowload data files.
                DataEntry::File(p) => {
                    let path = data_dir.join(p);
                    let root_url = "https://raw.githubusercontent.com/Yucklys/wudao/main/";
                    download_file(format!("{}{}", root_url, p), path)
                        .expect(&format!("Failed to download {} from GitHub Repo.", p));
                }
            }
        }

        Ok(())
    }

    /// Check if any data file is missing.
    pub fn check<'a>() -> Vec<DataEntry<'a>> {
        let mut missing = vec![];

        // Check if root dir exist.
        let data_dir = data_dir().unwrap();
        if !data_dir.exists() {
            missing.push(DataEntry::Dir(""));
        }

        // Check if changelog.txt exist.
        let changelog_file = data_dir.join("changelog.txt");
        if !changelog_file.exists() {
            missing.push(DataEntry::File("changelog.txt"));
        }

        missing
    }
}

#[derive(Debug)]
pub enum DataEntry<'a> {
    Dir(&'a str),
    File(&'a str),
}

fn data_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "yucklys", "Wu Dao") {
        Some(proj_dirs.data_dir().to_path_buf())
    } else {
        None
    }
}

fn download_file(url: String, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.error_for_status();
    match response {
        Ok(r) => {
            let content = r.text()?;
            let mut file = fs::File::create(path)?;
            io::copy(&mut content.as_bytes(), &mut file)?;
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

/// Read content from files
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let data_dir = data_dir().unwrap();
    let path = data_dir.join(path);
    std::fs::read_to_string(path)
}
