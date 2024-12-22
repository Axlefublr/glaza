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
mod models;
mod sh;

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
    let data = DataFiles::build(args.git)?;
    let current_model = CurrentRepo::try_from(data.current.as_path())?;
    let mut watched_model = WatchedRepo::try_from(data.watched.as_path())?;
    let mut wl_model = WlRepo::try_from(data.watch_later.as_path())?;
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
        UserCommands::Plink { show, web } => {
            let show = current_model.normalize_show_pattern(&show)?;
            if web {
                current_model.open_link(&show, false)?;
            } else {
                println!("{}", current_model.get_link(&show, false));
            }
            Ok(())
        },
        UserCommands::Pdlink { show, web } => {
            let show = current_model.normalize_show_pattern(&show)?;
            if web {
                current_model.open_link(&show, true)?;
            } else {
                println!("{}", current_model.get_link(&show, true));
            }
            Ok(())
        },
        UserCommands::Finish { show, grab, fresh } => {
            let show: String = if grab {
                let show = wl_model.normalize_show_pattern(&show)?;
                wl_model.remove(&show)?;
                show.into()
            } else if !fresh {
                let show = current_model.normalize_show_pattern(&show)?;
                current_model.remove(&show)?;
                show.into()
            } else {
                show
            };
            watched_model.finish(&show)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("finish -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Drop { show, grab, fresh } => {
            let latest_episode;
            let show: String = if grab {
                let show = wl_model.normalize_show_pattern(&show)?;
                latest_episode = current_model.get_episode(&show);
                wl_model.remove(&show)?;
                show.into()
            } else if !fresh {
                let show = current_model.normalize_show_pattern(&show)?;
                latest_episode = current_model.get_episode(&show);
                current_model.remove(&show)?;
                show.into()
            } else {
                latest_episode = 0;
                show
            };
            watched_model.drop(latest_episode, &show)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("drop at {latest_episode} -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Start {
            show,
            link,
            dlink,
            grab,
        } => {
            let show: String = if grab {
                let show = wl_model.normalize_show_pattern(&show)?;
                wl_model.remove(&show)?;
                show.into()
            } else {
                show
            };
            current_model.new_show(&show, link.as_ref(), dlink.as_ref())?;
            if args.git {
                git_add_commit(&data.data_dir, format!("start -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Shows { links } => Ok(current_model.list(links)?),
        UserCommands::Remove { show } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.remove(&show)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("remove -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Episode { show, episode } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_episode(&show, episode)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("watch ep{episode} -> {show}"))?
            }
            Ok(())
        },
        UserCommands::Download { show, episode } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_downloaded(&show, episode)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("download ep{episode} -> {show}"))?;
            }
            Ok(())
        },
        UserCommands::Link { show, link } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_link(&show, &link, false)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("update link -> {show} -> {link}"))?
            }
            Ok(())
        },
        UserCommands::Dlink { show, link } => {
            let show = current_model.normalize_show_pattern(&show)?;
            current_model.change_link(&show, &link, true)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("update dlink -> {show} -> {link}"))?
            }
            Ok(())
        },
        UserCommands::Add { show } => {
            wl_model.add(&show)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("wl add -> {}", show))?;
            }
            Ok(())
        },
        UserCommands::Discard { show } => {
            let show = wl_model.normalize_show_pattern(&show)?;
            wl_model.remove(&show)?;
            if args.git {
                git_add_commit(&data.data_dir, format!("wl remove -> {}", show))?;
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
