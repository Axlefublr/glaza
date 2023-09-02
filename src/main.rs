use crate::args::Args;
use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use show::SetActions;
use show::ShowCommands;

mod args;
mod data;
mod git;
mod show;
mod shows_model;
mod wl;

fn main() {
    let args = Args::parse();
    let data = DataFiles::create();
    let mut shows_model = shows_model::new(&data.shows);
    match args.action {
        UserCommands::Show { action } => match action {
            ShowCommands::Set { action } => match action {
                SetActions::Download { show, episode } => {
                    shows_model::change_downloaded(&mut shows_model, &show, episode);
                    shows_model::save(shows_model, &data.shows);
                }
                SetActions::Episode { show, episode } => {
                    shows_model::change_episode(&mut shows_model, &show, episode);
                    shows_model::save(shows_model, &data.shows);
                    // todo: commits are done after a singular save at the end by setting a variable to a variant of an enum or smth
                    git::add_commit(
                        &data.floral_barrel,
                        &data.shows,
                        format!("watch ep{episode} -> {show}"),
                    );
                }
                SetActions::Link { show, link } => {
                    shows_model::change_link(&mut shows_model, &show, link);
                    shows_model::save(shows_model, &data.shows);
                }
            },
            ShowCommands::Watch { show } => {}
            ShowCommands::Download { show } => {}
            _ => (),
        },
        UserCommands::Wl { action } => {}
    };
}
