use args::UserCommands;
use clap::Parser;
use show::SetActions;
use show::ShowCommands;
use crate::args::Args as Args;

mod args;
mod show;
mod wl;
mod data;

fn main() {
	println!("Hello, world!");
}
