use crate::sh::open_in_browser;
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

impl Show {
	pub fn new(link: &str) -> Self {
		Self {
			episode: 0,
			downloaded: 0,
			link: link.to_owned(),
		}
	}
}

pub struct CurrentRepo {
	pub current: Shows,
	file_path: PathBuf,
}

impl CurrentRepo {
	pub fn new(file_path: &Path) -> Result<Self, &'static str> {
		let current = parse(file_path)?;
		Ok(Self {
			current,
			file_path: file_path.to_path_buf(),
		})
	}

	fn get_mut_show(&mut self, show_name: &str) -> Result<&mut Show, String> {
		match self.current.get_mut(show_name) {
			Some(show) => Ok(show),
			None => Err(format!("couldn't find show {show_name} in current model")),
		}
	}

	fn get_show(&self, show_name: &str) -> Result<&Show, String> {
		match self.current.get(show_name) {
			Some(show) => Ok(show),
			None => Err(format!("couldn't find show {show_name} in current model")),
		}
	}

	pub fn new_show(mut self, show_name: &str, link: &str) -> Result<(), String> {
		self.current.insert(show_name.to_owned(), Show::new(link));
		self.save()
	}

	pub fn list(&self, should_links: bool) -> Result<(), &'static str> {
		let longest_title = match self.current.keys().map(|show_name| show_name.len()).max() {
			Some(length) => length,
			None => return Err("you have no shows you're currently watching"),
		};
		// this unwrap is safe because we just confirmed the iterator wouldn't be empty
		let biggest_episode = self
			.current
			.values()
			.map(|show| show.episode.to_string().len())
			.max()
			.unwrap();
		let biggest_download = self
			.current
			.values()
			.map(|show| show.downloaded.to_string().len())
			.max()
			.unwrap();
		let mut link = String::from("");
		for (show_name, show_obj) in self.current.iter() {
			let title_diff = " ".repeat(longest_title - show_name.len());
			let episode_diff = " ".repeat(biggest_episode - show_obj.episode.to_string().len());
			let download_diff = " ".repeat(biggest_download - show_obj.downloaded.to_string().len());
			if should_links {
				link = format!(" - {}", show_obj.link);
			};
			println!(
				"{show_name}{title_diff} - ep{}{episode_diff} - dn{}{download_diff}{}",
				show_obj.episode, show_obj.downloaded, link
			);
		}
		Ok(())
	}

	pub fn remove(mut self, show_name: &str) -> Result<(), String> {
		if self.current.remove(show_name).is_none() {
			return Err(format!("couldn't find show {show_name}"));
		}
		self.save()
	}

	pub fn change_episode(mut self, show_name: &str, new_episode: u32) -> Result<(), String> {
		self.get_mut_show(show_name)?.episode = new_episode;
		self.save()
	}

	pub fn change_downloaded(mut self, show_name: &str, new_downloaded: u32) -> Result<(), String> {
		self.get_mut_show(show_name)?.downloaded = new_downloaded;
		self.save()
	}

	pub fn change_link(mut self, show_name: &str, new_link: &str) -> Result<(), String> {
		self.get_mut_show(show_name)?.link = new_link.to_owned();
		self.save()
	}

	pub fn get_next_episode_link(&self, show_name: &str) -> Result<String, String> {
		let show = self.get_show(show_name)?;
		Ok(format!("{}{}", show.link, show.episode + 1))
	}

	pub fn open_next_episode_link(&self, show_name: &str) -> Result<(), String> {
		open_in_browser(&self.get_next_episode_link(show_name)?)?;
		Ok(())
	}

	pub fn get_next_download_link(&self, show_name: &str) -> Result<String, String> {
		let show = self.get_show(show_name)?;
		Ok(format!("{}{}", show.link, show.downloaded + 1))
	}

	pub fn open_next_download_link(&self, show_name: &str) -> Result<(), String> {
		open_in_browser(&self.get_next_download_link(show_name)?)?;
		Ok(())
	}

	pub fn open_link(&self, show_name: &str) -> Result<(), String> {
		open_in_browser(&self.get_link(show_name)?)?;
		Ok(())
	}

	pub fn get_link(&self, show_name: &str) -> Result<String, String> {
		let show = self.get_show(show_name)?;
		Ok(show.link.to_string())
	}

	pub fn save(self) -> Result<(), String> {
		let formatter = PrettyFormatter::with_indent(b"	");
		let mut data = Vec::new();
		let mut serializer = Serializer::with_formatter(&mut data, formatter);
		if self.current.serialize(&mut serializer).is_err() {
			return Err("couldn't serialize current model into json".to_owned());
		};
		if fs::write(self.file_path, data).is_err() {
			return Err("failed to write to current.json".to_owned());
		}
		Ok(())
	}
}

fn parse(file_path: &Path) -> Result<Shows, &'static str> {
	let file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => return Err("couldn't open current.json for reading"),
	};
	let reader = BufReader::new(file);
	match serde_json::from_reader(reader) {
		Ok(model) => Ok(model),
		Err(_) => Err("couldn't parse current.json into model"),
	}
}
