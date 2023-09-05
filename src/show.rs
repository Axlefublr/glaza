use clap::Subcommand;

pub mod actions;

#[derive(Subcommand)]
// #[command(next_line_help = true)]
pub enum ShowCommands {
	/// Print the next episode link
	Watch {
		#[arg(short, long)]
		show: String,
		/// Open the link in your $BROWSER instead of printing it
		#[arg(short, long)]
		open: bool
	},
	/// Print the next download link
	#[command(visible_alias = "dn")]
	Download {
		#[arg(short, long)]
		show: String,
		/// Open the link in your $BROWSER instead of printing it
		#[arg(short, long)]
		open: bool
	},
	/// Print the link property of a show
	Link {
		#[arg(short, long)]
		show: String,
		/// Open the link in your $BROWSER instead of printing it
		#[arg(short, long)]
		open: bool
	},
	/// Finish a show, putting it in your watched list
	Finish {
		#[arg(short, long)]
		show: String,
	},
	/// Drop a show, putting it in your watched list
	Drop {
		#[arg(short, long)]
		show: String,
	},
	/// Start a new show
	New {
		#[arg(short, long)]
		show: String,
		#[arg(short, long, default_value_t = String::from(""))]
		link: String,
	},
	/// List all the shows you're currently watching
	List {
		#[arg(short, long)]
		links: bool,
	},
	/// Print the entirety of the watched list
	Past,
	/// Removes a show from the list without putting it in your watched list
	#[command(visible_alias = "rm")]
	Remove {
		#[arg(short, long)]
		show: String,
	},
	/// Update a show's properties
	Set {
		#[command(subcommand)]
		action: SetActions,
	},
}

#[derive(Subcommand)]
pub enum SetActions {
	/// Set the episode you just watched
	#[command(visible_alias = "ep")]
	Episode {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		episode: u32,
	},
	/// Set the episode you just downloaded
	#[command(visible_alias = "dn")]
	Download {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		episode: u32,
	},
	/// Set the link
	Link {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		link: String,
	},
}