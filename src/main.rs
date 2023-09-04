use crate::args::Args;
use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use git::git_add_commit;
use show::SetActions;
use show::ShowCommands;
use shows_model::ShowsRepo;
use std::process::ExitCode;
use watched_model::WatchedRepo;

mod args;
mod data;
mod git;
mod show;
mod shows_model;
mod watched_model;
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
			return ExitCode::FAILURE;
		}
	};
	let mut watched_model = match WatchedRepo::new(&data.watched) {
		Ok(watched_model) => watched_model,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	return match args.action {
		UserCommands::Show { action } => match action {
			ShowCommands::Set { action } => match action {
				SetActions::Download { show, episode } => {
					if let Err(message) = shows_model.change_downloaded(&show, episode) {
						eprintln!("{}", message);
						return ExitCode::FAILURE;
					};
					match shows_model.save() {
						Err(message) => {
							eprintln!("{}", message);
							ExitCode::FAILURE
						}
						Ok(()) => ExitCode::SUCCESS,
					}
				}
				SetActions::Episode { show, episode } => {
					if let Err(message) = shows_model.change_episode(&show, episode) {
						eprintln!("{}", message);
						return ExitCode::FAILURE;
					};
					if let Err(message) = shows_model.save() {
						eprintln!("{}", message);
						return ExitCode::FAILURE;
					}
					// todo: commits are done after a singular save at the end by setting a variable to a variant of an enum or smth
					if let Err(message) =
						git_add_commit(&data.floral_barrel, format!("watch ep{episode} -> {show}"))
					{
						eprintln!("{}", message);
						return ExitCode::FAILURE;
					}
					ExitCode::SUCCESS
				}
				SetActions::Link { show, link } => {
					if let Err(message) = shows_model.change_link(&show, link) {
						eprintln!("{}", message);
						return ExitCode::FAILURE;
					};
					match shows_model.save() {
						Err(message) => {
							eprintln!("{}", message);
							ExitCode::FAILURE
						}
						Ok(()) => ExitCode::SUCCESS,
					}
				}
			},
			ShowCommands::Watch { show } => {
				if let Err(message) = shows_model.print_next_episode_link(&show) {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				};
				ExitCode::SUCCESS
			}
			ShowCommands::Download { show } => {
				if let Err(message) = shows_model.print_next_download_link(&show) {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				ExitCode::SUCCESS
			}
			ShowCommands::Link { show } => {
				if let Err(message) = shows_model.print_link(&show) {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				ExitCode::SUCCESS
			}
			ShowCommands::Finish { show } => {
				let _ = shows_model.remove(&show);
				if let Err(message) = shows_model.save() {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				if let Err(message) = watched_model.finish(&show) {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				if let Err(message) = git_add_commit(&data.floral_barrel, format!("finish {show}"))
				{
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				ExitCode::SUCCESS
			}
			ShowCommands::Drop { show } => {
				let _ = shows_model.remove(&show);
				if let Err(message) = shows_model.save() {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				if let Err(message) = watched_model.drop(&show) {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				if let Err(message) = git_add_commit(&data.floral_barrel, format!("drop {show}")) {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				ExitCode::SUCCESS
			}
			ShowCommands::New { show, link } => {
				shows_model.new_show(show, link);
				if let Err(message) = shows_model.save() {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				ExitCode::SUCCESS
			}
			ShowCommands::List => {
				if let Err(message) = shows_model.list() {
					eprintln!("{}", message);
					return ExitCode::FAILURE;
				}
				ExitCode::SUCCESS
			}
			_ => unimplemented!(),
		},
		// UserCommands::Wl { action } => unimplemented!(),
		_ => unimplemented!(),
	};
}
