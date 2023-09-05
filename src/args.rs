use crate::show::ShowCommands;
use crate::wl::WlCommands;
use clap::Parser;
use clap::Subcommand;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
	#[command(subcommand)]
	pub action: UserCommands,
	/// If the action writes to a file, commit that change
	#[arg(short, long)]
	pub git: bool,
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
