use std::fs;
use std::path::Path;
use std::path::PathBuf;

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

    pub fn add(mut self, what: &str) -> Result<(), &'static str> {
        let mut lines: Vec<String> =
            self.contents.lines().map(|line| line.to_owned()).collect();
        lines.push(what.to_owned());
        self.contents = lines.join("\n");
        self.save()
    }

    pub fn remove(mut self, what: &str) -> Result<(), &'static str> {
        self.contents = self
            .contents
            .lines()
            .filter(|line| *line != what)
            .collect::<Vec<_>>()
            .join("\n");
        self.save()
    }

    pub fn list(&self) {
        println!("{}", self.contents.trim_end())
    }

    fn save(self) -> Result<(), &'static str> {
        fs::write(self.file_path, self.contents)
            .map_err(|_| "couldn't write to watch later file")
    }
}

pub fn parse(file_path: &Path) -> Result<String, &'static str> {
    fs::read_to_string(file_path).map_err(|_| "couldn't read watch later file")
}
