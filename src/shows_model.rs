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

type Shows = HashMap<String, Show>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
	pub episode: u32,
	pub downloaded: u32,
	pub link: String,
}

pub struct ShowsRepo {
	pub shows: Shows,
	pub file_path: PathBuf,
}

impl ShowsRepo {
	pub fn new(file_path: &Path) -> Result<Self, &'static str> {
		let shows = parse(file_path)?;
		Ok(Self {
			shows,
			file_path: file_path.to_path_buf(),
		})
	}

	fn get_mut_show(&mut self, show_name: &str) -> Result<&mut Show, String> {
		match self.shows.get_mut(show_name) {
			Some(show) => Ok(show),
			None => Err(format!("couldn't find show {show_name} in shows model")),
		}
	}

	fn get_show(&self, show_name: &str) -> Result<&Show, String> {
		match self.shows.get(show_name) {
			Some(show) => Ok(show),
			None => Err(format!("couldn't find show {show_name} in shows model")),
		}
	}

	pub fn remove(&mut self, show_name: &str) -> Result<(), String> {
		match self.shows.remove(show_name) {
			Some(_) => Ok(()),
			None => Err(format!("couldn't find show {show_name}"))
		}
	}

	pub fn change_episode(&mut self, show_name: &str, new_episode: u32) -> Result<(), String> {
		self.get_mut_show(show_name)?.episode = new_episode;
		Ok(())
	}

	pub fn change_downloaded(
		&mut self,
		show_name: &str,
		new_downloaded: u32,
	) -> Result<(), String> {
		self.get_mut_show(show_name)?.downloaded = new_downloaded;
		Ok(())
	}

	pub fn change_link(&mut self, show_name: &str, new_link: String) -> Result<(), String> {
		self.get_mut_show(show_name)?.link = new_link;
		Ok(())
	}

	pub fn print_next_episode_link(&self, show_name: &str) -> Result<(), String> {
		let show = self.get_show(show_name)?;
		println!("{}{}", show.link, show.episode + 1);
		Ok(())
	}

	pub fn print_next_download_link(&self, show_name: &str) -> Result<(), String> {
		let show = self.get_show(show_name)?;
		println!("{}{}", show.link, show.downloaded + 1);
		Ok(())
	}

	pub fn print_link(&self, show_name: &str) -> Result<(), String> {
		let show = self.get_show(show_name)?;
		println!("{}", show.link);
		Ok(())
	}

	pub fn save(self) -> Result<(), &'static str> {
		let formatter = PrettyFormatter::with_indent(b"	");
		let mut data = Vec::new();
		let mut serializer = Serializer::with_formatter(&mut data, formatter);
		if self.shows.serialize(&mut serializer).is_err() {
			return Err("couldn't serialize shows model into json");
		};
		if fs::write(self.file_path, data).is_err() {
			return Err("failed to write to shows.json");
		}
		Ok(())
	}
}

fn parse(file_path: &Path) -> Result<Shows, &'static str> {
	let file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => return Err("couldn't open the shows file for reading"),
	};
	let reader = BufReader::new(file);
	match serde_json::from_reader(reader) {
		Ok(model) => Ok(model),
		Err(_) => Err("couldn't parse shows.json into model"),
	}
}
