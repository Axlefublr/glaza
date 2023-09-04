use chrono::Utc;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct WatchedRepo {
	file: File,
}

impl WatchedRepo {
	pub fn new(file_path: &Path) -> Result<Self, &'static str> {
		let file = parse(file_path)?;
		Ok(Self { file })
	}

	pub fn read(&mut self) -> Result<(), &'static str> {
		let mut contents = String::new();
		if self.file.read_to_string(&mut contents).is_err() {
			return Err("couldn't read watched file");
		}
		println!("{}", contents.trim_end());
		Ok(())
	}

	fn append_show(&mut self, text: &str) -> Result<(), &'static str> {
		let today = Utc::now().format("%y.%m.%d");
		if writeln!(self.file, "{} - {}", today, text).is_err() {
			return Err("couldn't write to watched file");
		}
		Ok(())
	}

	pub fn finish(&mut self, show: &str) -> Result<(), &'static str> {
		self.append_show(show)
	}

	pub fn drop(&mut self, show: &str) -> Result<(), &'static str> {
		self.append_show(&format!("(dropped) {}", show))
	}
}

fn parse(file_path: &Path) -> Result<File, &'static str> {
	match OpenOptions::new().append(true).read(true).open(file_path) {
		Ok(file) => Ok(file),
		Err(_) => Err("couldn't open the watched file for appending"),
	}
}
