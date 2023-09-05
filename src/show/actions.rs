use crate::sh::git_add_commit;
use crate::shows_model::ShowsRepo;
use crate::watched_model::WatchedRepo;
use std::path::Path;
use std::process::ExitCode;

pub mod set;

pub fn watch(show: String, open: bool, shows_model: ShowsRepo) -> ExitCode {
	if open {
		if let Err(message) = shows_model.open_next_episode_link(&show) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		};
	} else {
		match shows_model.get_next_episode_link(&show) {
			Ok(link) => println!("{}", link),
			Err(message) => {
				eprintln!("{}", message);
				return ExitCode::FAILURE;
			}
		};
	}
	ExitCode::SUCCESS
}

pub fn download(show: String, open: bool, shows_model: ShowsRepo) -> ExitCode {
	if open {
		if let Err(message) = shows_model.open_next_download_link(&show) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	} else {
		match shows_model.get_next_download_link(&show) {
			Ok(link) => println!("{}", link),
			Err(message) => {
				eprintln!("{}", message);
				return ExitCode::FAILURE;
			}
		}
	}
	ExitCode::SUCCESS
}

pub fn link(show: String, open: bool, shows_model: ShowsRepo) -> ExitCode {
	if open {
		if let Err(message) = shows_model.open_link(&show) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	} else {
		match shows_model.get_link(&show) {
			Ok(link) => println!("{}", link),
			Err(message) => {
				eprintln!("{}", message);
				return ExitCode::FAILURE;
			}
		}
	}
	ExitCode::SUCCESS
}

pub fn finish(
	show: String,
	mut shows_model: ShowsRepo,
	mut watched_model: WatchedRepo,
	data_dir: &Path,
	should_commit: bool
) -> ExitCode {
	let _ = shows_model.remove(&show);
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if let Err(message) = watched_model.finish(&show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("finish -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn drop(
	show: String,
	mut shows_model: ShowsRepo,
	mut watched_model: WatchedRepo,
	data_dir: &Path,
	should_commit: bool
) -> ExitCode {
	let _ = shows_model.remove(&show);
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if let Err(message) = watched_model.drop(&show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("drop -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn new(show: String, link: String, mut shows_model: ShowsRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	shows_model.new_show(show.clone(), link);
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("start -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn list(links: bool, shows_model: ShowsRepo) -> ExitCode {
	if let Err(message) = shows_model.list(links) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn past(mut watched_model: WatchedRepo) -> ExitCode {
	if let Err(message) = watched_model.read() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn remove(show: String, mut shows_model: ShowsRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	if let Err(message) = shows_model.remove(&show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("remove -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}