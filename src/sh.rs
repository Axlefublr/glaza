use std::env;
use std::path::Path;
use std::process::Command;

pub fn git_add_commit(working_dir: &Path, message: String) -> Result<(), &'static str> {
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

pub fn open_in_browser(link: &str) -> Result<(), &'static str> {
    let browser = get_browser()?;
    Command::new(browser).arg(link).output().unwrap();
    Ok(())
}

pub fn is_git_init(data_dir: &Path) -> bool {
    data_dir.join(".git").exists()
}

pub fn git_init(data_dir: &Path) -> Result<(), String> {
    if Command::new("git")
        .arg("init")
        .current_dir(data_dir)
        .output()
        .is_err()
    {
        Err(format!(
            "couldn't git init the data directory in {}",
            data_dir.display()
        ))
    } else {
        Ok(())
    }
}

fn get_browser() -> Result<String, &'static str> {
    match env::var("BROWSER") {
        Ok(browser) => Ok(browser),
        Err(_) => Err("your $BROWSER environment variable is undefined"),
    }
}
