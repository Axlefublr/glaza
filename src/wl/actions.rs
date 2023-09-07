use super::model::WlRepo;
use crate::sh::git_add_commit;
use crate::show::model::ShowsRepo;
use std::path::Path;
use std::process::ExitCode;

pub fn add(show: &str, wl_model: WlRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	if let Err(message) = wl_model.add(show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("wl add -> {}", show)) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn remove(show: &str, wl_model: WlRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	if let Err(message) = wl_model.remove(show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("wl remove -> {}", show)) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}

pub fn list(wl_model: WlRepo) -> ExitCode {
	wl_model.list();
	ExitCode::SUCCESS
}

pub fn start(show: &str, link: &str, wl_model: WlRepo, shows_model: ShowsRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	if let Err(message) = wl_model.remove(show) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if let Err(message) = shows_model.new_show(show, link) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if should_commit {
		if let Err(message) = git_add_commit(data_dir, format!("start -> {}", show)) {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	}
	ExitCode::SUCCESS
}