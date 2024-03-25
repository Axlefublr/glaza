use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub action: UserCommands,
    /// If the action writes to a file, commit that change
    #[arg(short, long)]
    pub git:    bool,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Print the next episode's link.
    /// This works by appending the watched episode numer + 1 onto the link.
    /// This won't work if a number appended on the link doesn't result in that
    /// episode's url.
    #[command(visible_alias = "next")]
    Go {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Print the next download link.
    /// Works the same as the `go` subcommand, except the `saved` episode is
    /// appended instead.
    #[command(visible_alias = "dn")]
    Download {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Print the link of a show.
    /// This is most useful for shows that don't support `go` & `download` due to
    /// having non-standard urls.
    Where {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Finish a show, putting it in your watched list.
    Finish {
        show: String,
        /// Remove the show from the watch later list, if it's there.
        /// If it's not there, return an error.
        /// This is to help you realize if you misspelled a show title.
        #[arg(short, long)]
        grab: bool,
    },
    /// Drop a show, putting it in your watched list.
    /// The distinction from `finish` is that to the left of the show name in
    /// your watched list, there will be the `(dropped)` specifier.
    /// Also with the `--git` flag, the commit message will say "drop" instead of
    /// "finish".
    Drop {
        show: String,
        /// Remove the show from the watch later list, if it's there.
        /// If it's not there, return an error.
        /// This is to help you realize if you misspelled a show title.
        #[arg(short, long)]
        grab: bool,
    },
    /// Start a new show, putting it in your ‘currently watching’ list.
    #[command(visible_alias = "new")]
    Start {
        show: String,
        /// Optional link to where you're going to be watching the show.
        /// If you want to make use of the `go` & `download` features, cut the
        /// link so that if you appended a number after it, you'd get the
        /// link to that episode. Not all links work like that, in which
        /// case the features will be unavailable.
        #[arg(default_value_t = String::from(""))]
        link: String,
        /// Remove the show from the watch later list, if it's there.
        /// If it's not there, return an error.
        /// This is to help you realize if you misspelled a show title.
        #[arg(short, long)]
        grab: bool,
    },
    /// List all the shows you're currently watching.
    Shows {
        /// Display the links of each show as well.
        #[arg(short, long)]
        links: bool,
    },
    /// Remove a show from the list without putting it in your watched list.
    /// This is useful if you accidentally added a show you didn't mean to,
    /// possibly due to misspelling its title.
    #[command(visible_alias = "rm")]
    #[command(visible_alias = "delete")]
    Remove { show: String },
    /// Set the episode you just watched.
    #[command(visible_alias = "ep")]
    Episode { show: String, episode: u32 },
    /// Set the episode you just downloaded.
    #[command(visible_alias = "sv")]
    Save { show: String, episode: u32 },
    /// Update the link of a show.
    Link { show: String, link: String },
    /// Add a new show to your watch later list.
    Add { show: String },
    /// Remove a show from your watch later list.
    Discard { show: String },
    /// Print the entire contents of your watch later file.
    Wl,
    /// Print the entire contents of your watched list.
    #[command(visible_alias = "past")]
    Watched,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Args;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert()
    }
}
