use super::Args;
use clap::CommandFactory;

#[test]
fn verify_cli() {
    Args::command().debug_assert()
}
