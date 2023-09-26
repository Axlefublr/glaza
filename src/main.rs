use crate::args::Args;
use args::UserCommands;
use clap::Parser;
use data::DataFiles;
use show::model::CurrentRepo;
use show::SetActions;
use show::ShowCommands;
use std::process::ExitCode;
use watched_model::WatchedRepo;
use wl::model::WlRepo;
use wl::WlCommands;

mod args;
mod data;
mod sh;
mod show;
mod watched_model;
mod wl;

fn main() -> ExitCode {
	let args = Args::parse();
	let data = DataFiles::new();
	if let Err(message) = data.create(args.git) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	let current_model = match CurrentRepo::new(&data.current) {
		Ok(current_model) => current_model,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let watched_model = match WatchedRepo::new(&data.watched) {
		Ok(watched_model) => watched_model,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let wl_model = match WlRepo::new(&data.watch_later) {
		Ok(wl_model) => wl_model,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	match args.action {
		UserCommands::Show { action } => match action {
			ShowCommands::Set { action } => match action {
				SetActions::Download { show, episode } => {
					show::actions::set::download(&show, episode, current_model, &data.floral_barrel, args.git)
				}
				SetActions::Episode { show, episode } => {
					show::actions::set::episode(&show, episode, current_model, &data.floral_barrel, args.git)
				}
				SetActions::Link { show, link } => {
					show::actions::set::link(&show, &link, current_model, &data.floral_barrel, args.git)
				}
			},
			ShowCommands::Watch { show, open } => show::actions::watch(&show, open, current_model),
			ShowCommands::Download { show, open } => show::actions::download(&show, open, current_model),
			ShowCommands::Link { show, open } => show::actions::link(&show, open, current_model),
			ShowCommands::Finish { show } => {
				show::actions::finish(&show, current_model, watched_model, &data.floral_barrel, args.git)
			}
			ShowCommands::Drop { show } => {
				show::actions::drop(&show, current_model, watched_model, &data.floral_barrel, args.git)
			}
			ShowCommands::New { show, link } => {
				show::actions::new(&show, &link, current_model, &data.floral_barrel, args.git)
			}
			ShowCommands::List { links } => show::actions::list(links, current_model),
			ShowCommands::Past => show::actions::past(watched_model),
			ShowCommands::Remove { show } => {
				show::actions::remove(&show, current_model, &data.floral_barrel, args.git)
			}
		},
		UserCommands::Wl { action } => match action {
			WlCommands::Add { show } => wl::actions::add(&show, wl_model, &data.floral_barrel, args.git),
			WlCommands::List => wl::actions::list(wl_model),
			WlCommands::Remove { show } => {
				wl::actions::remove(&show, wl_model, &data.floral_barrel, args.git)
			}
			WlCommands::Start { show, link } => wl::actions::start(
				&show,
				&link,
				wl_model,
				current_model,
				&data.floral_barrel,
				args.git,
			),
		},
	}
}
