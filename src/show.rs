use clap::Subcommand;

#[derive(Subcommand)]
pub enum ShowCommands {
	/// Open a show you're currently watching in your $BROWSER.
	/// Works by concatenating the link you specified for the show and the episode you last set with `floral_barrel show set episode` + 1, effectively resulting in "watch the next episode".
	Watch {
		#[arg(short, long)]
		show: String
	},
	/// Open a show you're currently watching in your $BROWSER, for downloading.
	/// Works by concatenating the link you specified for the show and the episode you last set with `floral_barrel show set download` + 1, effectively resulting in "download the next episode"
	Download {
		#[arg(short, long)]
		show: String
	},
	/// Finish a show, putting it in your watched list with the date of finishing.
	Finish {

	},
	/// Drop a show, putting it in your watched list tagged as dropped, with the date of dropping.
	Drop {

	},
	/// Start a new show by specifying a link to it.
	/// For `show watch` and `show download` to work properly, strip the end of the link so when you append a number to it, it results in the correct link to the episode.
	New {

	},
	/// List all the shows you're currently watching in the format of `showName - ep99 - dn99`
	List {

	},
	/// Print the entirety of the watched file, effectively showing all shows you've ever watched.
	Past {

	},
	/// Update how many episodes of a show you're currently watching you've watched / downloaded.
	Set {
		#[command(subcommand)]
		action: SetActions
	},
}

#[derive(Subcommand)]
pub enum SetActions {
	/// Set the episode you just watched of a show
	Episode {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		episode: u32
	},
	/// Set the episode you just downloaded of a show
	Download {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		episode: u32
	},
	Link {
		#[arg(short, long)]
		show: String,
		#[arg(short, long)]
		link: String
	}
}