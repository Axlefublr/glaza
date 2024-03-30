use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::ser::Serializer;

use crate::sh::open_in_browser;

use super::ValidatedTitle;

type Shows = HashMap<String, Show>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub episode:    u32,
    pub downloaded: u32,
    pub link:       String,
}

impl Show {
    pub fn new(link: &str) -> Self {
        Self {
            episode:    0,
            downloaded: 0,
            link:       link.to_owned(),
        }
    }
}

pub struct CurrentRepo {
    current:   Shows,
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

    pub fn normalize_show_pattern(&self, pattern: &str) -> Result<ValidatedTitle, String> {
        let lowercase_pattern = pattern.to_lowercase();

        for key in self.current.keys() {
            if key == pattern {
                eprintln!("successful exact case-sensitive match: {}", key);
                return Ok(ValidatedTitle(key.to_owned()));
            }
        }

        for key in self.current.keys() {
            if key.to_lowercase() == lowercase_pattern {
                eprintln!("successful exact case-insensitive match: {}", key);
                return Ok(ValidatedTitle(key.to_owned()));
            }
        }

        let mut candidates: Vec<_> = self
            .current
            .keys()
            .filter(|&show| show.to_lowercase().contains(&lowercase_pattern))
            .collect();

        if candidates.is_empty() {
            return Err("unsuccessful case-insensitive substring match".into());
        }

        let insensitive_candidates = candidates.clone();
        let mut retained = false;
        if candidates.len() > 1 {
            candidates.retain(|&show| show.contains(pattern));
            retained = true;
        }

        match candidates.len() {
            0 => Err(format!(
            "case-insensitive substring match (too many): '{}'\nand then, unsuccessful case-sensitive substring match",
                insensitive_candidates
                    .iter()
                    .map(|candidate| *candidate as &str)
                    .collect::<Vec<&str>>()
                    .join("', '")
            )),
            1 => {
                // if we got here without retaining, that means we matched precisely a single show
                // case-insensitively.
                // if we did retain, that means we could only get to precisely 1 show once we
                // searched case-sensitively
                eprintln!("successful case-{}sensitive substring match: '{}'", if retained { "" } else { "in" } , candidates[0]);
                Ok(ValidatedTitle(candidates[0].to_owned()))
            },
            _ => Err(format!(
                "case-insensitive substring match (too many): '{}'\nand then, case-sensitive substring match (too many): '{}'",
                insensitive_candidates
                    .iter()
                    .map(|candidate| *candidate as &str)
                    .collect::<Vec<&str>>()
                    .join("', '"),
                candidates
                    .iter()
                    .map(|candidate| *candidate as &str)
                    .collect::<Vec<&str>>()
                    .join("', '")
            )),
        }
    }

    fn get_mut_show(&mut self, show_title: &ValidatedTitle) -> &mut Show {
        self.current
            .get_mut(&show_title.0)
            .unwrap()
    }

    fn get_show(&self, show_title: &ValidatedTitle) -> &Show {
        self.current.get(&show_title.0).unwrap()
    }

    pub fn new_show(&mut self, show_title: &str, link: &str) -> Result<(), String> {
        self.current.insert(show_title.to_owned(), Show::new(link));
        self.save()
    }

    pub fn list(&self, should_links: bool) -> Result<(), &'static str> {
        let longest_title = match self.current.keys().map(|show_name| show_name.len()).max() {
            Some(length) => length,
            None => return Err("you have no shows you're currently watching"),
        };
        // this unwrap is safe because we just confirmed the iterator wouldn't be
        // empty
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
        for (show_title, show_obj) in self.current.iter() {
            let title_diff = " ".repeat(longest_title - show_title.len());
            let episode_diff = " ".repeat(biggest_episode - show_obj.episode.to_string().len());
            let download_diff = " ".repeat(biggest_download - show_obj.downloaded.to_string().len());
            if should_links {
                link = format!(" - {}", show_obj.link);
            };
            println!(
                "{show_title}{title_diff} - ep{}{episode_diff} - dn{}{download_diff}{}",
                show_obj.episode, show_obj.downloaded, link
            );
        }
        Ok(())
    }

    pub fn remove(&mut self, show_title: &ValidatedTitle) -> Result<(), String> {
        self.current.remove(&show_title.0).unwrap();
        self.save()
    }

    pub fn change_episode(&mut self, show_title: &ValidatedTitle, new_episode: u32) -> Result<(), String> {
        self.get_mut_show(show_title).episode = new_episode;
        self.save()
    }

    pub fn change_downloaded(&mut self, show_title: &ValidatedTitle, new_downloaded: u32) -> Result<(), String> {
        self.get_mut_show(show_title).downloaded = new_downloaded;
        self.save()
    }

    pub fn change_link(&mut self, show_title: &ValidatedTitle, new_link: &str) -> Result<(), String> {
        self.get_mut_show(show_title).link = new_link.to_owned();
        self.save()
    }

    pub fn get_next_episode_link(&self, show_title: &ValidatedTitle) -> String {
        let show = self.get_show(show_title);
        format!("{}{}", show.link, show.episode + 1)
    }

    pub fn open_next_episode_link(&self, show_title: &ValidatedTitle) -> Result<(), String> {
        open_in_browser(&self.get_next_episode_link(show_title))?;
        Ok(())
    }

    pub fn get_next_download_link(&self, show_title: &ValidatedTitle) -> String {
        let show = self.get_show(show_title);
        format!("{}{}", show.link, show.downloaded + 1)
    }

    pub fn open_next_download_link(&self, show_title: &ValidatedTitle) -> Result<(), String> {
        open_in_browser(&self.get_next_download_link(show_title))?;
        Ok(())
    }

    pub fn open_link(&self, show_title: &ValidatedTitle) -> Result<(), String> {
        open_in_browser(&self.get_link(show_title))?;
        Ok(())
    }

    pub fn get_link(&self, show_title: &ValidatedTitle) -> String {
        let show = self.get_show(show_title);
        show.link.to_string()
    }

    pub fn save(&mut self) -> Result<(), String> {
        let formatter = PrettyFormatter::with_indent(b"	");
        let mut data = Vec::new();
        let mut serializer = Serializer::with_formatter(&mut data, formatter);
        if self.current.serialize(&mut serializer).is_err() {
            return Err("couldn't serialize current model into json".to_owned());
        };
        if fs::write(&self.file_path, data).is_err() {
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
