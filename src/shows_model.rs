use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::ser::Serializer;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub type Shows = HashMap<String, Show>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub episode: u32,
    pub downloaded: u32,
    pub link: String,
}

pub fn new(file_path: &Path) -> Shows {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn change_episode(shows: &mut Shows, show_name: &str, new_episode: u32) {
    shows.get_mut(show_name).unwrap().episode = new_episode;
}

pub fn change_downloaded(shows: &mut Shows, show_name: &str, new_downloaded: u32) {
    shows.get_mut(show_name).unwrap().downloaded = new_downloaded;
}

pub fn change_link(shows: &mut Shows, show_name: &str, new_link: String) {
    shows.get_mut(show_name).unwrap().link = new_link;
}

pub fn save(shows_model: Shows, shows_path: &Path) {
    let formatter = PrettyFormatter::with_indent(b"	"); // tab
    let mut vec = Vec::new();
    let mut serializer = Serializer::with_formatter(&mut vec, formatter);
    shows_model.serialize(&mut serializer).unwrap();
    fs::write(shows_path, vec).unwrap();
}
