use std::env;
use std::path::Path;
use std::process::Command;

pub fn add_commit(working_dir: &Path, file: &Path, message: String) {
	Command::new("git")
		.arg("add")
		.arg(file)
		.current_dir(working_dir)
		.output()
		.unwrap();
	Command::new("git")
		.arg("commit")
		.arg("-m")
		.arg(message)
		.current_dir(working_dir)
		.output()
		.unwrap();
}
