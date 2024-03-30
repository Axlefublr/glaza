use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::sh;
use crate::sh::is_git_init;

const EMPTY_JSON_OBJECT: &str = r"{}";
const DATA_DIR: &str = "glaza";
const SHOWS_FILE: &str = "current.json";
const WATCHED_FILE: &str = "watched.txt";
const WATCH_LATER_FILE: &str = "watch-later.txt";

fn get_glaza_dir() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap();
    data_dir.join(DATA_DIR)
}

fn file_exists(file_path: &Path) -> bool {
    let metadata = fs::metadata(file_path);
    metadata.is_ok() && metadata.unwrap().is_file()
}

fn ensure_exists(file_path: &Path, contents: &str) -> Result<(), String> {
    if file_exists(file_path) {
        return Ok(());
    }
    if fs::write(file_path, contents).is_err() {
        return Err(format!("failed to create file {}", file_path.display()));
    }
    Ok(())
}

#[derive(Debug)]
pub struct DataFiles {
    pub current:     PathBuf,
    pub watched:     PathBuf,
    pub watch_later: PathBuf,
    pub glaza:       PathBuf,
}

impl DataFiles {
    pub fn new() -> Self {
        let glaza = get_glaza_dir();
        let current = glaza.join(SHOWS_FILE);
        let watched = glaza.join(WATCHED_FILE);
        let watch_later = glaza.join(WATCH_LATER_FILE);
        Self {
            current,
            watched,
            watch_later,
            glaza,
        }
    }

    pub fn create(&self, git_init: bool) -> Result<(), String> {
        fs::create_dir_all(&self.glaza).map_err(|_| format!("couldn't create {}", &self.glaza.display()))?;
        if git_init && !is_git_init(&self.glaza) {
            sh::git_init(&self.glaza)?;
        }
        ensure_exists(&self.current, EMPTY_JSON_OBJECT)?;
        ensure_exists(&self.watched, "")?;
        ensure_exists(&self.watch_later, "")?;
        Ok(())
    }
}
