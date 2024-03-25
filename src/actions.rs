use std::error::Error;
use std::path::Path;

use super::models::wl::WlRepo;
use crate::models::show::CurrentRepo;
use crate::models::watched::WatchedRepo;
use crate::sh::git_add_commit;

pub fn show_set_downloaded(
    show: &str,
    episode: u32,
    current_model: CurrentRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    current_model.change_downloaded(show, episode)?;
    if should_commit {
        git_add_commit(data_dir, format!("download ep{episode} -> {show}"))?;
    }
    Ok(())
}

pub fn show_set_episode(
    show: &str,
    episode: u32,
    current_model: CurrentRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    current_model.change_episode(show, episode)?;
    if should_commit {
        git_add_commit(data_dir, format!("watch ep{episode} -> {show}"))?
    }
    Ok(())
}

pub fn show_set_link(
    show: &str,
    link: &str,
    current_model: CurrentRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    current_model.change_link(show, link)?;
    if should_commit {
        git_add_commit(data_dir, format!("update link -> {show} -> {link}"))?
    }
    Ok(())
}

pub fn wl_add(
    show: &str,
    wl_model: WlRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    wl_model.add(show)?;
    if should_commit {
        git_add_commit(data_dir, format!("wl add -> {}", show))?;
    }
    Ok(())
}

pub fn wl_discard(
    show: &str,
    wl_model: WlRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    wl_model.remove(show)?;
    if should_commit {
        git_add_commit(data_dir, format!("wl remove -> {}", show))?;
    }
    Ok(())
}

pub fn wl_list(wl_model: WlRepo) -> Result<(), Box<dyn Error>> {
    wl_model.list();
    Ok(())
}

// pub fn start(
//     show: &str,
//     link: &str,
//     wl_model: WlRepo,
//     current_model: CurrentRepo,
//     data_dir: &Path,
//     should_commit: bool,
// ) -> ExitCode {
//     if let Err(message) = wl_model.remove(show) {
//         eprintln!("{}", message);
//         return ExitCode::FAILURE;
//     };
//     if let Err(message) = current_model.new_show(show, link) {
//         eprintln!("{}", message);
//         return ExitCode::FAILURE;
//     }
//     if should_commit {
//         if let Err(message) =
//             git_add_commit(data_dir, format!("start -> {}", show))
//         {
//             eprintln!("{}", message);
//             return ExitCode::FAILURE;
//         }
//     }
//     ExitCode::SUCCESS
// }

pub fn show_get_episode(
    show: &str,
    web: bool,
    current_model: CurrentRepo,
) -> Result<(), Box<dyn Error>> {
    if web {
        current_model.open_next_episode_link(show)?;
    } else {
        eprintln!("{}", current_model.get_next_episode_link(show)?);
    }
    Ok(())
}

pub fn show_get_download(
    show: &str,
    web: bool,
    current_model: CurrentRepo,
) -> Result<(), Box<dyn Error>> {
    if web {
        current_model.open_next_download_link(show)?;
    } else {
        eprintln!("{}", current_model.get_next_download_link(show)?);
    }
    Ok(())
}

pub fn show_get_link(
    show: &str,
    web: bool,
    current_model: CurrentRepo,
) -> Result<(), Box<dyn Error>> {
    if web {
        current_model.open_link(show)?;
    } else {
        println!("{}", current_model.get_link(show)?);
    }
    Ok(())
}

pub fn show_finish(
    show: &str,
    grab: bool,
    current_model: CurrentRepo,
    mut watched_model: WatchedRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    let _ = current_model.remove(show);
    watched_model.finish(show)?;
    if should_commit {
        git_add_commit(data_dir, format!("finish -> {show}"))?;
    }
    Ok(())
}

pub fn show_drop(
    show: &str,
    grab: bool,
    current_model: CurrentRepo,
    mut watched_model: WatchedRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    let _ = current_model.remove(show);
    watched_model.drop(show)?;
    if should_commit {
        git_add_commit(data_dir, format!("drop -> {show}"))?;
    }
    Ok(())
}

pub fn show_start(
    show: &str,
    link: &str,
    grab: bool,
    current_model: CurrentRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    current_model.new_show(show, link)?;
    if should_commit {
        git_add_commit(data_dir, format!("start -> {show}"))?;
    }
    Ok(())
}

pub fn show_list(
    links: bool,
    current_model: CurrentRepo,
) -> Result<(), Box<dyn Error>> {
    Ok(current_model.list(links)?)
}

pub fn watched_list(mut watched_model: WatchedRepo) -> Result<(), Box<dyn Error>> {
    Ok(watched_model.read()?)
}

pub fn show_remove(
    show: &str,
    current_model: CurrentRepo,
    data_dir: &Path,
    should_commit: bool,
) -> Result<(), Box<dyn Error>> {
    current_model.remove(show)?;
    if should_commit {
        git_add_commit(data_dir, format!("remove -> {show}"))?;
    }
    Ok(())
}
