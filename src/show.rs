use clap::Subcommand;

#[derive(Subcommand)]
pub enum ShowCommands {
	/// Print the link of a show you're currently watching to stdout.
	/// Works by concatenating the link you specified for the show and the episode you last set with `floral_barrel show set episode` + 1, effectively resulting in "print the next episode's link".
	Watch {
		#[arg(short, long)]
		show: String,
	},
	/// Print the link of a show you're currently watching to stdout, for downloading.
	/// Works by concatenating the link you specified for the show and the episode you last set with `floral_barrel show set download` + 1, effectively resulting in "print the download link for the next episode"
	Download {
		#[arg(short, long)]
		show: String,
	},
	/// Print the link of a show to stdout.
	/// Compared to `show watch` and `show download`, no useful magic is done.
	/// This is useful for streaming services that don't conveniently have the episode number as the last thing in the link.
	Link {
		#[arg(short, long)]
		show: String,
	},
	/// Finish a show, putting it in your watched list with the date of finishing.
	Finish {
		#[arg(short, long)]
		show: String,
	},
	/// Drop a show, putting it in your watched list tagged as dropped, with the date of dropping.
	Drop {
		#[arg(short, long)]
		show: String,
	},
	/// Start a new show by specifying a link to it.
	/// For `show watch` and `show download` to work properly, strip the end of the link so when you append a number to it, it results in the correct link to the episode.
	New {},
	/// List all the shows you're currently watching in the format of `showName - ep99 - dn99`
	List {},
	/// Print the entirety of the watched file, effectively showing all shows you've ever watched.
	Past {},
	/// Update how many episodes of a show you're currently watching you've watched / downloaded.
	Set {
		#[command(subcommand)]
		action: SetActions,
	},
}

#[derive(Subcommand)]
pub enum SetActions {
	/// Set the episode you just watched of a show
	Episode {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		episode: u32,
	},
	/// Set the episode you just downloaded of a show
	Download {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		episode: u32,
	},
	Link {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		link: String,
	},
}
