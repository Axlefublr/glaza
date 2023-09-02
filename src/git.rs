use std::path::Path;
use std::process::Command;

pub fn add_commit(working_dir: &Path, file: &Path, message: String) {
	let command = format!(
		"git add {} && git commit -m \"{}\"",
		file.display(),
		message
	);

	Command::new("sh")
		.arg("-c")
		.arg(&command)
		.current_dir(working_dir)
		.output()
		.unwrap();
}
