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

fn main() {
	let args = Args::parse();
	let data = DataFiles::create();
	match args.action {
		UserCommands::Show { action } => match action {
			ShowCommands::Set { action } => match action {
				SetActions::Download { show, episode } => {

				},
				SetActions::Episode { show, episode } => {

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
}
