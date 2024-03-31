use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::ValidatedTitle;
use crate::sh::open_in_browser;

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
    pub fn normalize_show_pattern(&self, pattern: &str) -> Result<ValidatedTitle, String> {
        ValidatedTitle::from_pattern(self.current.keys().cloned().collect::<Vec<String>>(), pattern)
    }

    fn get_mut_show(&mut self, show_title: &ValidatedTitle) -> &mut Show {
        self.current.get_mut(&show_title.0).unwrap()
    }

    fn get_show(&self, show_title: &ValidatedTitle) -> &Show {
        self.current.get(&show_title.0).unwrap()
    }

    pub fn new_show(&mut self, show_title: &str, link: &str) -> Result<(), &'static str> {
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
                link = format!(" — {}", show_obj.link);
            };
            println!(
                "{show_title}{title_diff} — ep{}{episode_diff} — dn{}{download_diff}{}",
                show_obj.episode, show_obj.downloaded, link
            );
        }
        Ok(())
    }

    pub fn remove(&mut self, show_title: &ValidatedTitle) -> Result<(), &'static str> {
        self.current.remove(&show_title.0).unwrap();
        self.save()
    }

    pub fn change_episode(
        &mut self,
        show_title: &ValidatedTitle,
        new_episode: u32,
    ) -> Result<(), &'static str> {
        self.get_mut_show(show_title).episode = new_episode;
        self.save()
    }

    pub fn change_downloaded(
        &mut self,
        show_title: &ValidatedTitle,
        new_downloaded: u32,
    ) -> Result<(), &'static str> {
        self.get_mut_show(show_title).downloaded = new_downloaded;
        self.save()
    }

    pub fn change_link(&mut self, show_title: &ValidatedTitle, new_link: &str) -> Result<(), &'static str> {
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

    pub fn save(&mut self) -> Result<(), &'static str> {
        let yaml =
            serde_yaml::to_string(&self.current).map_err(|_| "couldn't serialize current model into yaml")?;
        fs::write(self.file_path.as_path(), yaml).map_err(|_| "failed to write to current.yml") // we ensure the file exists on creation of the type
    }
}

impl TryFrom<&Path> for CurrentRepo {
    type Error = &'static str;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file_path)
            .map_err(|_| "could not create and/or open current.yml for reading")?;
        let reader = BufReader::new(file);
        let current =
            serde_yaml::from_reader(reader).map_err(|_| "couldn't deserialize current.yml into model")?;
        Ok(Self {
            current,
            file_path: file_path.to_path_buf(),
        })
    }
}
