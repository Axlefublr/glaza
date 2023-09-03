use crate::show::ShowCommands;
use crate::wl::WlCommands;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, about)]
pub struct Args {
	#[command(subcommand)]
	pub action: UserCommands,
	/// Indentation you want to use for your shows.json file
	#[arg(long, default_value_t = String::from("	"))]
	pub indent: String,
}

#[derive(Subcommand)]
pub enum UserCommands {
	/// Commands to interact with the shows you're currently watching or have watched
	Show {
		#[command(subcommand)]
		action: ShowCommands,
	},
	/// Commands to interact with your "Watch later" list
	Wl {
		#[command(subcommand)]
		action: WlCommands,
	},
}
