use crate::sh::git_add_commit;
use crate::show::model::CurrentRepo;
use crate::watched_model::WatchedRepo;
use std::path::Path;
use std::process::ExitCode;

pub mod set;

pub fn watch(show: &str, open: bool, current_model: CurrentRepo) -> ExitCode {
	if open {
		if let Err(message) = current_model.open_next_episode_link(show) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		};
	} else {
		match current_model.get_next_episode_link(show) {
			Ok(link) => println!("{}", link),
			Err(message) => {
				eprintln!("{}", message);
				return ExitCode::FAILURE;
			}
		};
	}
	ExitCode::SUCCESS
}

pub fn download(show: &str, open: bool, current_model: CurrentRepo) -> ExitCode {
	if open {
		if let Err(message) = current_model.open_next_download_link(show) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	} else {
		match current_model.get_next_download_link(show) {
			Ok(link) => println!("{}", link),
			Err(message) => {
				eprintln!("{}", message);
				return ExitCode::FAILURE;
			}
		}
	}
	ExitCode::SUCCESS
}

pub fn link(show: &str, open: bool, current_model: CurrentRepo) -> ExitCode {
	if open {
		if let Err(message) = current_model.open_link(show) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	} else {
		match current_model.get_link(show) {
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
	show: &str,
	current_model: CurrentRepo,
	mut watched_model: WatchedRepo,
	data_dir: &Path,
	should_commit: bool,
) -> ExitCode {
	let _ = current_model.remove(show);
	if let Err(message) = watched_model.finish(show) {
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
	show: &str,
	current_model: CurrentRepo,
	mut watched_model: WatchedRepo,
	data_dir: &Path,
	should_commit: bool,
) -> ExitCode {
	let _ = current_model.remove(show);
	if let Err(message) = watched_model.drop(show) {
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

pub fn new(
	show: &str,
	link: &str,
	current_model: CurrentRepo,
	data_dir: &Path,
	should_commit: bool,
) -> ExitCode {
	if let Err(message) = current_model.new_show(show, link) {
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

pub fn list(links: bool, current_model: CurrentRepo) -> ExitCode {
	if let Err(message) = current_model.list(links) {
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

pub fn remove(show: &str, current_model: CurrentRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	if let Err(message) = current_model.remove(show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("remove -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}
