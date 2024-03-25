use std::error::Error;

use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use models::show::CurrentRepo;
use models::watched::WatchedRepo;
use models::wl::WlRepo;

use crate::args::Args;

mod actions;
mod args;
mod data;
mod sh;
mod models {
    pub mod show;
    pub mod watched;
    pub mod wl;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let data = DataFiles::new();
    data.create(args.git)?;
    let current_model = CurrentRepo::new(&data.current)?;
    let watched_model = WatchedRepo::new(&data.watched)?;
    let wl_model = WlRepo::new(&data.watch_later)?;
    match args.action {
        UserCommands::Go { show, web } => {
            actions::show_get_episode(&show, web, current_model)
        },
        UserCommands::Download { show, web } => {
            actions::show_get_download(&show, web, current_model)
        },
        UserCommands::Where { show, web } => {
            actions::show_get_link(&show, web, current_model)
        },
        UserCommands::Finish { show, grab } => actions::show_finish(
            &show,
            grab,
            current_model,
            watched_model,
            &data.glaza,
            args.git,
        ),
        UserCommands::Drop { show, grab } => actions::show_drop(
            &show,
            grab,
            current_model,
            watched_model,
            &data.glaza,
            args.git,
        ),
        UserCommands::Start { show, link, grab } => actions::show_start(
            &show,
            &link,
            grab,
            current_model,
            &data.glaza,
            args.git,
        ),
        UserCommands::Shows { links } => actions::show_list(links, current_model),
        UserCommands::Remove { show } => {
            actions::show_remove(&show, current_model, &data.glaza, args.git)
        },
        UserCommands::Episode { show, episode } => actions::show_set_episode(
            &show,
            episode,
            current_model,
            &data.glaza,
            args.git,
        ),
        UserCommands::Save { show, episode } => actions::show_set_downloaded(
            &show,
            episode,
            current_model,
            &data.glaza,
            args.git,
        ),
        UserCommands::Link { show, link } => actions::show_set_link(
            &show,
            &link,
            current_model,
            &data.glaza,
            args.git,
        ),
        UserCommands::Add { show } => {
            actions::wl_add(&show, wl_model, &data.glaza, args.git)
        },
        UserCommands::Discard { show } => {
            actions::wl_discard(&show, wl_model, &data.glaza, args.git)
        },
        UserCommands::Wl => actions::wl_list(wl_model),
        UserCommands::Watched => actions::watched_list(watched_model),
    }
}
