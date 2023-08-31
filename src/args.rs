use clap::Parser;
use clap::Subcommand;
use crate::list::ListCommands;
use crate::set::SetCommands;
use crate::show::ShowCommands;

#[derive(Parser)]
#[command(author, about, next_line_help = true)]
pub struct Args {
	#[command(subcommand)]
	pub action: UserCommands
}

#[derive(Subcommand, Clone, Copy)]
pub enum UserCommands {
	Show {
		#[command(subcommand)]
		action: ShowCommands
	},
	Set {
		#[command(subcommand)]
		action: SetCommands
	},
	List {
		#[command(subcommand)]
		action: ListCommands
	}
}