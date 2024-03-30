use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::ValidatedTitle;

pub struct WlRepo {
    contents:  String,
    file_path: PathBuf,
}

impl WlRepo {
    pub fn new(file_path: &Path) -> Result<Self, &'static str> {
        let contents = parse(file_path)?;
        Ok(Self {
            file_path: file_path.to_path_buf(),
            contents,
        })
    }

    pub fn normalize_show_pattern(&self, pattern: &str) -> Result<ValidatedTitle, String> {
        ValidatedTitle::from_pattern(self.contents.lines().map(|line| line.to_owned()).collect(), pattern)
    }

    pub fn add(&mut self, what: &str) -> Result<(), String> {
        if self.contents.lines().any(|line| line == what) {
            return Err(format!("watch later list already contains: '{}'", what))
        }
        let mut lines: Vec<String> = self.contents.lines().map(|line| line.to_owned()).collect();
        lines.push(what.to_owned());
        self.contents = lines.join("\n");
        Ok(self.save()?)
    }

    pub fn remove(&mut self, show_title: &ValidatedTitle) -> Result<(), &'static str> {
        self.contents = self
            .contents
            .lines()
            .filter(|line| *line != show_title.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        self.save()
    }

    pub fn list(&self) {
        println!("{}", self.contents.trim_end())
    }

    fn save(&mut self) -> Result<(), &'static str> {
        fs::write(&self.file_path, &self.contents).map_err(|_| "couldn't write to watch later file")
    }
}

pub fn parse(file_path: &Path) -> Result<String, &'static str> {
    fs::read_to_string(file_path).map_err(|_| "couldn't read watch later file")
}
