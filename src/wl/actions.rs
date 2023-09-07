use super::model::WlRepo;
use crate::sh::git_add_commit;
use std::path::Path;
use std::process::ExitCode;

pub fn add(show: String, wl_model: WlRepo, data_dir: &Path, should_commit: bool) -> ExitCode {
	if let Err(message) = wl_model.add(&show) {
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
