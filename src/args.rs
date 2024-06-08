use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version)]
#[command(about = std::include_str!("description.txt"))]
pub struct Args {
    #[command(subcommand)]
    pub action: UserCommands,
    /// If the action writes to a file, commit that change
    #[arg(short, long)]
    pub git:    bool,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List all the shows you're currently watching.
    #[command(visible_alias = "s")]
    Shows {
        /// Display the links of each show as well.
        #[arg(short, long)]
        links: bool,
    },
    /// Print the next episode's link.
    /// This works by appending the watched episode numer + 1 onto the link.
    /// This won't work if a number appended on the link doesn't result in that
    /// episode's url.
    #[command(visible_alias = "next")]
    #[command(visible_alias = "go")]
    #[command(visible_alias = "w")]
    Watch {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Print the next download link.
    /// Works the same as the `watch` subcommand, except the `saved` episode is
    /// appended instead.
    #[command(visible_alias = "install")]
    #[command(visible_alias = "i")]
    Save {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Print the episode link of a show.
    /// This is most useful for shows that don't support `watch` due to
    /// having non-standard urls.
    #[command(visible_alias = "h")]
    Plink {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Print the download link of a show.
    /// This is most useful for shows that don't support `save` due to
    /// having non-standard urls.
    Pdlink {
        show: String,
        /// Open the link in your $BROWSER instead of printing it.
        #[arg(short, long)]
        web:  bool,
    },
    /// Set the episode you just watched.
    #[command(visible_alias = "ep")]
    Episode { show: String, episode: u32 },
    /// Set the episode you just downloaded.
    #[command(visible_alias = "dn")]
    Download { show: String, episode: u32 },
    /// Start a new show, putting it in your ‘currently watching’ list.
    #[command(visible_alias = "new")]
    #[command(visible_alias = "n")]
    /// Update the episode link of a show.
    /// It will be used for the `watch` and `plink` subcommands.
    /// And also, as a fallback if you don't define a download link.
    #[command(visible_alias = "ln")]
    Link { show: String, link: String },
    /// Update the download link of a show.
    /// It will be used for the `save` and `pdlink` subcommands.
    /// And also, as a fallback if you don't define an episode link.
    Dlink { show: String, link: String },
    Start {
        show:  String,
        /// Optional link to where you're going to be watching the show.
        /// If you want to make use of the `watch` subcommand, cut the
        /// link so that if you appended a number after it, you'd get the
        /// link to that episode. Not all links work like that, in which
        /// case the feature will be unavailable.
        /// If this link is the only one set, it will be used as a fallback
        /// for when subcommands expect a download link.
        #[arg(short, long)]
        link:  Option<String>,
        /// Same as the `link` flag, but for the download link instead.
        /// This link is used for the `save` subcommand.
        /// If this link is the only one set, it will be used as a fallback
        /// for when subcommands expect an episode link.
        #[arg(short, long)]
        dlink: Option<String>,
        /// Remove the show from the watch later list, if it's there.
        /// If it's not there, return an error.
        /// This is to help you realize if you misspelled a show title.
        #[arg(short, long)]
        grab:  bool,
    },
    /// Finish a show, putting it in your watched list.
    #[command(visible_alias = "f")]
    Finish {
        show:  String,
        /// Remove the show from the watch later list, instead of the current list.
        /// If it's not there, return an error.
        /// This is to help you realize if you misspelled a show title.
        #[arg(short, long)]
        grab:  bool,
        /// Ignore the current list and take the show title literally.
        /// This flag is like doing `start` and then `finish` immediately.
        /// Useful for movies, where you generally start and finish a "show" at the same time,
        /// where adding it to the current list with `start` first makes no sense.
        /// If `--grab`/`-g` is specified, this flag is ignored.
        #[arg(short, long)]
        fresh: bool,
    },
    /// Drop a show, putting it in your watched list.
    /// The distinction from `finish` is that to the left of the show name in
    /// your watched list, there will be the `(dropped)` specifier.
    /// Also with the `--git` flag, the commit message will say "drop" instead of
    /// "finish".
    #[command(visible_alias = "d")]
    Drop {
        show:  String,
        /// Remove the show from the watch later list, instead of the current list.
        /// If it's not there, return an error.
        /// This is to help you realize if you misspelled a show title.
        #[arg(short, long)]
        grab:  bool,
        /// Ignore the current list and take the show title literally.
        /// This flag is like doing `start` and then `finish` immediately.
        /// Useful for movies, where you generally start and finish a "show" at the same time,
        /// where adding it to the current list with `start` first makes no sense.
        /// If `--grab`/`-g` is specified, this flag is ignored.
        #[arg(short, long)]
        fresh: bool,
    },
    /// Remove a show from the list without putting it in your watched list.
    /// This is useful if you accidentally added a show you didn't mean to,
    /// possibly due to misspelling its title.
    #[command(visible_alias = "rm")]
    #[command(visible_alias = "delete")]
    Remove { show: String },
    /// Add a new show to your watch later list.
    #[command(visible_alias = "later")]
    #[command(visible_alias = "a")]
    Add { show: String },
    /// Remove a show from your watch later list.
    #[command(visible_alias = "c")]
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
