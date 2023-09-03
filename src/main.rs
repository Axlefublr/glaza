use std::process::ExitCode;

use crate::args::Args;
use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use show::SetActions;
use show::ShowCommands;
use shows_model::ShowsRepo;

mod args;
mod data;
mod git;
mod show;
mod shows_model;
mod wl;

fn main() -> ExitCode {
	let args = Args::parse();
	let data = DataFiles::new();
	if let Err(message) = data.create() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	let mut shows_model = match ShowsRepo::new(&data.shows) {
		Ok(shows_model) => shows_model,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE
		}
	};
	match args.action {
		UserCommands::Show { action } => match action {
			ShowCommands::Set { action } => match action {
				SetActions::Download { show, episode } => {
					shows_model.change_downloaded(&show, episode);
					shows_model.save();
				}
				SetActions::Episode { show, episode } => {
					shows_model.change_episode(&show, episode);
					shows_model.save();
					// todo: commits are done after a singular save at the end by setting a variable to a variant of an enum or smth
					git::git_add_commit(
						&data.floral_barrel,
						&data.shows,
						format!("watch ep{episode} -> {show}"),
					);
				}
				SetActions::Link { show, link } => {
					shows_model.change_link(&show, link);
					shows_model.save();
				}
			},
			ShowCommands::Watch { show } => {}
			ShowCommands::Download { show } => {}
			_ => (),
		},
		UserCommands::Wl { action } => {}
	};
	ExitCode::SUCCESS
}
