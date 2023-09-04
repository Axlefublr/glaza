use clap::CommandFactory;
use super::Args;

#[test]
fn verify_cli() {
	Args::command().debug_assert()
}
