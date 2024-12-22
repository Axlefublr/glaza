use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::sh;
use crate::sh::is_git_init;

const DATA_DIR: &str = "glaza";
const CURRENT_FILE: &str = "current.yml";
const WATCHED_FILE: &str = "watched.txt";
const WATCH_LATER_FILE: &str = "watch-later.txt";

#[derive(Debug)]
pub struct DataFiles {
    pub current: PathBuf,
    pub watched: PathBuf,
    pub watch_later: PathBuf,
    pub data_dir: PathBuf,
}

impl DataFiles {
    pub fn build(git_init: bool) -> Result<Self, String> {
        let data_dir = get_data_dir();
        init_data_dir(data_dir.as_path(), git_init)?;
        let current = data_dir.join(CURRENT_FILE);
        let watched = data_dir.join(WATCHED_FILE);
        let watch_later = data_dir.join(WATCH_LATER_FILE);
        Ok(Self {
            current,
            watched,
            watch_later,
            data_dir,
        })
    }
}

fn get_data_dir() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap();
    data_dir.join(DATA_DIR)
}

fn init_data_dir(data_dir: &Path, git_init: bool) -> Result<(), String> {
    fs::create_dir_all(data_dir).map_err(|_| {
        format!(
            "couldn't create data directory as this path: {}",
            data_dir.display()
        )
    })?;
    if git_init && !is_git_init(data_dir) {
        sh::git_init(data_dir)?;
    }
    Ok(())
}
