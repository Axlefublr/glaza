use crate::args::Args;
use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use show::SetActions;
use show::ShowCommands;

mod args;
mod data;
mod show;
mod shows_model;
mod wl;
mod git;

fn main() {
	let args = Args::parse();
	let data = DataFiles::create();
	let mut shows_model = shows_model::new(&data.shows);
	match args.action {
		UserCommands::Show { action } => match action {
			ShowCommands::Set { action } => match action {
				SetActions::Download { show, episode } => {
					shows_model::change_downloaded(&mut shows_model, &show, episode);
				}
				SetActions::Episode { show, episode } => {
					shows_model::change_episode(&mut shows_model, &show, episode);
					git::add_commit(&data.floral_barrel, &data.shows, format!("watch ep{episode} -> {show}"));
				}
				SetActions::Link { show, link } => {
					shows_model::change_link(&mut shows_model, &show, link);
				}
			},
			ShowCommands::Watch { show } => {}
			ShowCommands::Download { show } => {}
			_ => (),
		},
		UserCommands::Wl { action } => {}
	};
	shows_model::save(shows_model, &data.shows);
}
