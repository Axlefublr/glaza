use dirs;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

const EMPTY_JSON_OBJECT: &str = r"{}";
const DATA_DIR: &str = "floral_barrel";
const SHOWS_FILE: &str = "shows.json";
const WATCHED_FILE: &str = "watched.txt";
const WATCH_LATER_FILE: &str = "watch-later.txt";

fn get_floral_barrel_dir() -> PathBuf {
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
	pub shows: PathBuf,
	pub watched: PathBuf,
	pub watch_later: PathBuf,
	pub floral_barrel: PathBuf,
}

impl DataFiles {
	pub fn new() -> Self {
		let floral_barrel = get_floral_barrel_dir();
		let shows = floral_barrel.join(SHOWS_FILE);
		let watched = floral_barrel.join(WATCHED_FILE);
		let watch_later = floral_barrel.join(WATCH_LATER_FILE);
		Self {
			shows,
			watched,
			watch_later,
			floral_barrel,
		}
	}

	pub fn create(&self) -> Result<(), String> {
		if fs::create_dir_all(&self.floral_barrel).is_err() {
			return Err(format!("couldn't create {}", &self.floral_barrel.display()));
		}
		ensure_exists(&self.shows, EMPTY_JSON_OBJECT)?;
		ensure_exists(&self.watched, "")?;
		ensure_exists(&self.watch_later, "")?;
		Ok(())
	}
}
