use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

use super::ValidatedTitle;
use crate::sh::open_in_browser;

type Shows = HashMap<String, Show>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub episode:    u32,
    pub downloaded: u32,
    pub link:       Option<String>,
    pub dlink:      Option<String>,
}

impl Show {
    pub fn new(link: Option<&String>, dlink: Option<&String>) -> Self {
        Self {
            episode:    0,
            downloaded: 0,
            link:       link.map(|value| value.to_owned()),
            dlink:      dlink.map(|value| value.to_owned()),
        }
    }

    fn resolve_link(&self, dlink_priority: bool) -> String {
        if dlink_priority {
            if let Some(dlink) = self.dlink.as_ref() {
                return dlink.to_owned();
            } else if let Some(link) = self.link.as_ref() {
                return link.to_owned();
            }
        } else if let Some(link) = self.link.as_ref() {
            return link.to_owned();
        } else if let Some(dlink) = self.dlink.as_ref() {
            return dlink.to_owned();
        }
        Default::default()
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

    pub fn new_show(
        mut self,
        show_title: &str,
        link: Option<&String>,
        dlink: Option<&String>,
    ) -> Result<(), &'static str> {
        self.current.insert(show_title.to_owned(), Show::new(link, dlink));
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
        for (show_title, show_obj) in self.current.iter() {
            if !should_links {
                let title_diff = " ".repeat(longest_title - show_title.len());
                let episode_diff = " ".repeat(biggest_episode - show_obj.episode.to_string().len());
                println!(
                    "{show_title}{title_diff} — ep{}{episode_diff} — dn{}",
                    show_obj.episode, show_obj.downloaded
                );
            } else {
                const LONG_SEPARATOR: &str = "  ";
                println!(
                    "{show_title} — ep{} — dn{}",
                    show_obj.episode, show_obj.downloaded
                );
                if let Some(link) = show_obj.link.as_ref() {
                    println!("{0}link: {1}", LONG_SEPARATOR, link);
                } else {
                    println!("{0}link: empty", LONG_SEPARATOR);
                }
                if let Some(dlink) = show_obj.dlink.as_ref() {
                    println!("{0}dlink: {1}", LONG_SEPARATOR, dlink);
                } else {
                    println!("{0}dlink: empty", LONG_SEPARATOR);
                }
            };
        }
        Ok(())
    }

    pub fn remove(mut self, show_title: &ValidatedTitle) -> Result<(), &'static str> {
        self.current.remove(&show_title.0).unwrap();
        self.save()
    }

    pub fn change_episode(
        mut self,
        show_title: &ValidatedTitle,
        new_episode: u32,
    ) -> Result<(), &'static str> {
        self.get_mut_show(show_title).episode = new_episode;
        self.save()
    }

    pub fn change_downloaded(
        mut self,
        show_title: &ValidatedTitle,
        new_downloaded: u32,
    ) -> Result<(), &'static str> {
        self.get_mut_show(show_title).downloaded = new_downloaded;
        self.save()
    }

    pub fn change_link(
        mut self,
        show_title: &ValidatedTitle,
        new_link: &str,
        is_dlink: bool,
    ) -> Result<(), &'static str> {
        if is_dlink {
            self.get_mut_show(show_title).dlink = Some(new_link.to_owned());
        } else {
            self.get_mut_show(show_title).link = Some(new_link.to_owned());
        }
        self.save()
    }

    pub fn get_episode(&self, show_title: &ValidatedTitle) -> u32 {
        self.get_show(show_title).episode
    }

    pub fn get_downloaded(&self, show_title: &ValidatedTitle) -> u32 {
        self.get_show(show_title).downloaded
    }

    pub fn get_next_episode_link(&self, show_title: &ValidatedTitle) -> String {
        let show = self.get_show(show_title);
        format!("{}{}", show.resolve_link(false), show.episode + 1)
    }

    pub fn open_next_episode_link(&self, show_title: &ValidatedTitle) -> Result<(), String> {
        open_in_browser(&self.get_next_episode_link(show_title))?;
        Ok(())
    }

    pub fn get_next_download_link(&self, show_title: &ValidatedTitle) -> String {
        let show = self.get_show(show_title);
        format!("{}{}", show.resolve_link(true), show.downloaded + 1)
    }

    pub fn open_next_download_link(&self, show_title: &ValidatedTitle) -> Result<(), String> {
        open_in_browser(&self.get_next_download_link(show_title))?;
        Ok(())
    }

    pub fn open_link(&self, show_title: &ValidatedTitle, dlink: bool) -> Result<(), String> {
        open_in_browser(&self.get_link(show_title, dlink))?;
        Ok(())
    }

    pub fn get_link(&self, show_title: &ValidatedTitle, dlink: bool) -> String {
        let show = self.get_show(show_title);
        show.resolve_link(dlink).to_owned()
    }

    pub fn save(self) -> Result<(), &'static str> {
        let mut entries: Vec<(String, Show)> = self.current.into_iter().collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        let mut sorted_shows = IndexMap::new();
        for (key, value) in entries {
            sorted_shows.insert(key, value);
        }

        let yaml =
            serde_yaml::to_string(&sorted_shows).map_err(|_| "couldn't serialize current model into yaml")?;
        fs::write(self.file_path.as_path(), yaml).map_err(|_| "failed to write to current.yml") // we ensure the file exists on creation of the type
    }
}

impl TryFrom<&Path> for CurrentRepo {
    type Error = &'static str;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
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
