use std::error::Error;
use std::process::ExitCode;

use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use models::show::CurrentRepo;
use models::watched::WatchedRepo;
use models::wl::WlRepo;
use sh::git_add_commit;

use crate::args::Args;

mod args;
mod data;
mod sh;
mod models;

fn main() -> ExitCode {
    match _main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        },
    }
}

fn _main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let data = DataFiles::new();
    data.create(args.git)?;
    let mut current_model = CurrentRepo::new(&data.current)?;
    let mut watched_model = WatchedRepo::new(&data.watched)?;
    let wl_model = WlRepo::new(&data.watch_later)?;
    match args.action {
        UserCommands::Watch { show, web } => {
            let show = current_model.normalize_show_pattern(&show)?;
            if web {
                current_model.open_next_episode_link(&show)?;
            } else {
                println!("{}", current_model.get_next_episode_link(&show));
            }
            Ok(())
        },
        UserCommands::Save { show, web } => {
            let show = current_model.normalize_show_pattern(&show)?;
            if web {
                current_model.open_next_download_link(&show)?;
            } else {
                println!("{}", current_model.get_next_download_link(&show));
            }
            Ok(())
        },
        UserCommands::Where { show, web } => {
            let show = current_model.normalize_show_pattern(&show)?;
            if web {
                current_model.open_link(&show)?;
            } else {
                println!("{}", current_model.get_link(&show));
            }
            Ok(())
        },
        UserCommands::Finish { show, grab, fresh } => {
            let show: String = if grab {
                wl_model.remove(&show)?;
                Default::default()
            } else if !fresh {
                let show = current_model.normalize_show_pattern(&show)?;
                current_model.remove(&show)?;
                show.into()
            } else {
                show
            };
            watched_model.finish(&show)?;
            if args.git {
                git_add_commit(&data.glaza, format!("finish -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Drop { show, grab, fresh } => {
            let show: String = if grab {
                wl_model.remove(&show)?;
                Default::default()
            } else if !fresh {
                let show = current_model.normalize_show_pattern(&show)?;
                current_model.remove(&show)?;
                show.into()
            } else {
                show
            };
            watched_model.drop(&show)?;
            if args.git {
                git_add_commit(&data.glaza, format!("drop -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Start { show, link, grab } => {
            let show: &str = if grab {
                wl_model.remove(&show)?;
                ""
            } else {
                &show
            };
            current_model.new_show(show, &link)?;
            if args.git {
                git_add_commit(&data.glaza, format!("start -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Shows { links } => Ok(current_model.list(links)?),
        UserCommands::Remove { show } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.remove(&show)?;
            if args.git {
                git_add_commit(&data.glaza, format!("remove -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Episode { show, episode } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_episode(&show, episode)?;
            if args.git {
                git_add_commit(&data.glaza, format!("watch ep{episode} -> {show}"))?
            }
            Ok(())
        },
        UserCommands::Download { show, episode } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_downloaded(&show, episode)?;
            if args.git {
                git_add_commit(&data.glaza, format!("download ep{episode} -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Link { show, link } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_link(&show, &link)?;
            if args.git {
                git_add_commit(&data.glaza, format!("update link -> {show} -> {link}"))?
            }
            Ok(())
        },
        UserCommands::Add { show } => {
            wl_model.add(&show)?;
            if args.git {
                git_add_commit(&data.glaza, format!("wl add -> {}", show))?;
            }
            Ok(())
        },
        UserCommands::Discard { show } => {
            wl_model.remove(&show)?;
            if args.git {
                git_add_commit(&data.glaza, format!("wl remove -> {}", show))?;
            }
            Ok(())
        },
        UserCommands::Wl => {
            wl_model.list();
            Ok(())
        },
        UserCommands::Watched => Ok(watched_model.read()?),
    }
}
