use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use super::ValidatedTitle;

pub struct WlRepo {
    contents: String,
    file_path: PathBuf,
}

impl WlRepo {
    pub fn normalize_show_pattern(&self, pattern: &str) -> Result<ValidatedTitle, String> {
        ValidatedTitle::from_pattern(
            self.contents.lines().map(|line| line.to_owned()).collect(),
            pattern,
        )
    }

    pub fn add(&mut self, what: &str) -> Result<(), String> {
        if self.contents.lines().any(|line| line == what) {
            return Err(format!("watch later list already contains: '{}'", what));
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

impl TryFrom<&Path> for WlRepo {
    type Error = &'static str;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .read(true)
            .open(file_path)
            .map_err(|_| "couldn't create and/or open the watch later file")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| "couldn't read watch later file, despite it existing")?;
        Ok(Self {
            file_path: file_path.to_path_buf(),
            contents,
        })
    }
}
