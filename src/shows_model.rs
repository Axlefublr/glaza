use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::ser::Serializer;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

const JSON_INDENT: &[u8] = b"	";

type Shows = HashMap<String, Show>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
	pub episode: u32,
	pub downloaded: u32,
	pub link: String,
}

pub struct ShowsRepo {
	pub shows: Shows,
	pub file_path: PathBuf
}

impl ShowsRepo {
	pub fn new(file_path: &Path) -> Result<Self, &'static str> {
		let shows = parse(file_path)?;
		Ok(Self { shows, file_path: file_path.to_path_buf() })
	}

	pub fn change_episode(&mut self, show_name: &str, new_episode: u32) {
		self.shows.get_mut(show_name).unwrap().episode = new_episode;
	}

	pub fn change_downloaded(&mut self, show_name: &str, new_downloaded: u32) {
		self.shows.get_mut(show_name).unwrap().downloaded = new_downloaded;
	}

	pub fn change_link(&mut self, show_name: &str, new_link: String) {
		self.shows.get_mut(show_name).unwrap().link = new_link;
	}

	pub fn save(self) -> Result<(), &'static str> {
		let formatter = PrettyFormatter::with_indent(JSON_INDENT); // todo: program flag to override json indentation
		let mut data = Vec::new();
		let mut serializer = Serializer::with_formatter(&mut data, formatter);
		if self.shows.serialize(&mut serializer).is_err() {
			return Err("couldn't serialize shows model into json");
		};
		if fs::write(self.file_path, data).is_err() {
			return Err("failed to write to shows.json")
		}
		Ok(())
	}

}

fn parse(file_path: &Path) -> Result<Shows, &'static str> {
	let file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => return Err("couldn't open the shows file for reading")
	};
	let reader = BufReader::new(file);
	match serde_json::from_reader(reader) {
		Ok(model) => Ok(model),
		Err(_) => Err("couldn't parse shows.json into model")
	}
}