use std::path::Path;
use std::process::Command;

pub fn git_add_commit(
	working_dir: &Path,
	message: String,
) -> Result<(), &'static str> {
	if Command::new("git")
		.arg("add")
		.arg(".")
		.current_dir(working_dir)
		.output()
		.is_err()
	{
		return Err("couldn't git add");
	}
	if Command::new("git")
		.arg("commit")
		.arg("-m")
		.arg(message)
		.current_dir(working_dir)
		.output()
		.is_err()
	{
		return Err("couldn't git commit");
	}
	Ok(())
}
