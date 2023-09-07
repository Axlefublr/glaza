use crate::sh::git_add_commit;
use crate::show::model::ShowsRepo;
use std::path::Path;
use std::process::ExitCode;

pub fn download(
	show: String,
	episode: u32,
	mut shows_model: ShowsRepo,
	data_dir: &Path,
	should_commit: bool,
) -> ExitCode {
	if let Err(message) = shows_model.change_downloaded(&show, episode) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("download ep{episode} -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn episode(
	show: String,
	episode: u32,
	mut shows_model: ShowsRepo,
	data_dir: &Path,
	should_commit: bool,
) -> ExitCode {
	if let Err(message) = shows_model.change_episode(&show, episode) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("watch ep{episode} -> {show}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn link(
	show: String,
	link: String,
	mut shows_model: ShowsRepo,
	data_dir: &Path,
	should_commit: bool,
) -> ExitCode {
	if let Err(message) = shows_model.change_link(&show, link.clone()) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if let Err(message) = shows_model.save() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("update link -> {show} -> {link}")) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}
