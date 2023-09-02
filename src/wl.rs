use clap::Subcommand;

#[derive(Subcommand, Clone, Copy)]
pub enum WlCommands {
	/// Add a new show to your watch later list
	Add {},
	/// Remove a show from your watch later list
	Remove {},
	/// Remove a show from your watch later list, and start watching it.
	/// This is like running `wl remove showName` and `show new showName link` in one command
	Start {},
	/// Print the entire contents of your watch later file
	List {},
}
