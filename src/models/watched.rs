use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use chrono::Utc;

pub struct WatchedRepo {
    file: File,
}

impl WatchedRepo {
    pub fn read(&mut self) -> Result<(), &'static str> {
        let mut contents = String::new();
        if self.file.read_to_string(&mut contents).is_err() {
            return Err("couldn't read watched file");
        }
        println!("{}", contents.trim_end());
        Ok(())
    }

    fn append_show(&mut self, show: &str) -> Result<(), &'static str> {
        let today = Utc::now().format("%y.%m.%d");
        if writeln!(self.file, "{} - {}", today, show).is_err() {
            return Err("couldn't append to watched file");
        }
        Ok(())
    }

    pub fn finish(&mut self, show: &str) -> Result<(), &'static str> {
        self.append_show(show)
    }

    pub fn drop(&mut self, latest_episode: u32, show: &str) -> Result<(), &'static str> {
        self.append_show(&format!("(dropped at ep {}) {}", latest_episode, show))
    }
}

impl TryFrom<&Path> for WatchedRepo {
    type Error = &'static str;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(file_path)
            .map_err(|_| "could not create and/or open the watched file")?;
        Ok(Self { file })
    }
}
