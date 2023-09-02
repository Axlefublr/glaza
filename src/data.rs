use dirs;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

const DATA_DIR: &str = "floral_barrel";
const SHOWS_FILE: &str = "shows.json";
const WATCHED_FILE: &str = "watched.txt";
const WATCH_LATER_FILE: &str = "watch-later.txt";

fn get_floral_barrel_dir() -> PathBuf {
	let data_dir = dirs::data_dir().unwrap();
	data_dir.join(DATA_DIR)
}

fn create(parent_buf: &Path, file_path: &str) -> PathBuf {
	let file = parent_buf.join(file_path);
	// todo! create file
	file
}

#[derive(Debug)]
pub struct DataFiles {
	pub shows: PathBuf,
	pub watched: PathBuf,
	pub watch_later: PathBuf,
	pub floral_barrel: PathBuf,
}

impl DataFiles {
	pub fn create() -> Self {
		let floral_barrel = get_floral_barrel_dir();
		let shows = create(&floral_barrel, SHOWS_FILE);
		let watched = create(&floral_barrel, WATCHED_FILE);
		let watch_later = create(&floral_barrel, WATCH_LATER_FILE);
		Self {
			shows,
			watched,
			watch_later,
			floral_barrel,
		}
	}
}
