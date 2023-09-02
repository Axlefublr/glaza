use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use show::SetActions;
use show::ShowCommands;
use crate::args::Args as Args;

mod args;
mod show;
mod wl;
mod data;
mod shows_model;

fn main() {
	let args = Args::parse();
	let data = DataFiles::create();
	let mut shows_model = shows_model::new(&data.shows);
	match args.action {
		UserCommands::Show { action } => match action {
			ShowCommands::Set { action } => match action {
				SetActions::Download { show, episode } => {
					shows_model::change_downloaded(&mut shows_model, &show, episode);
				},
				SetActions::Episode { show, episode } => {
					shows_model::change_episode(&mut shows_model, &show, episode);
				},
				SetActions::Link { show, link } => {
					shows_model::change_link(&mut shows_model, &show, link);
				}
			},
			ShowCommands::Watch { show } => {

			},
			ShowCommands::Download { show } => {

			},
			_ => ()
		},
		UserCommands::Wl { action } => {

		}
	};
	shows_model::save(shows_model, &data.shows);
}
