# glaza

> ‘glaza’ means ‘eyes’ in russian. It's also what you use to watch shows!

Track the shows you watch using your terminal!

`glaza` will create a directory where it will store its files.

Every time you specify the `-g`/`--git` flag, the action you do will be commited, if it writes to one of the files.

If you never specify the `--git` flag, the data directory won't be git initialized.

## The works

```fish
glaza --git add 'One Piece Live Action'
```

Add show with title 'One Piece Live Action' to the watch later list and commit that change.

The commit message will look like this: `wl add -> One Piece Live Action`

```fish
glaza -g start --grab 'live action' -l 'https://website.com/one-piece-live-action-1-'
```

Since we already have the full show title in our watch later list, we don't have to enter it fully again.

By using the `--grab`/`-g` flag, we pattern match 'One Piece Live Action' with the pattern 'live action'.

Instead of the `--git` flag, we're using the short option `-g`. These are equivalent.

With the `--link`/`-l` flag, we add a link that we intend to watch the show on.

If the link where you intend to _download_ the show on is different, you can also add it with the `--dlink`/`-d` flag.

Let's say now we've downloaded 6 episodes, and watched 3. Let's mark that using the `episode` and `download` subcommands:

```fish
glaza -g dn act 6
glaza -g ep act 3
```

Almost all subcommands have some sort of alias that you can use instead of the full name. Here, `dn` means `download` and `ep` means `episode`. To see all available subcommand aliases, view `glaza --help` (which is also included [at the end of this readme](#usage)).

Also, the pattern matching string 'act' should still be unique to match 'One Piece Live Action'!

Often times on streaming services, the url of a page ends in the episode number. When this is the case, remove the episode number when adding the link in `glaza`.

Here, the link would usually be this: `https://website.com/one-piece-live-action-1-1`

Supply this to the `--link`/`-l` (or `--dlink`/`-d`) flag instead: `https://website.com/one-piece-live-action-1-`

This will unlock the feature of the `watch` and `save` subcommands, that make use of this common pattern. They let you print / open in the `$BROWSER` the _next_ episode to watch / download.

We've marked that we watched 3 episodes with a command shown above. Let's open the 4th episode in the browser!

```fish
glaza watch 'live action' --web
```

Since we don't write to any of the data files here, and instead just retrieve some info, we don't need the `--git`/`-g` flag here.

If we didn't use the `--web`/`-w` flag, the link would just get printed to the console.

In that case, you might see output like this:

```
successful case-insensitive substring match: 'One Piece Live Action'
https://website.com/one-piece-live-action-1-4
```

You might find it annoying that the output isn't _just_ the link in this case. Well, I'm happy to announce that the first line there is actually printed to stderr, meaning that you can redirect it to `/dev/null` if you need just the link:

```fish
glaza watch 'live action' 2>/dev/null
```

> keep in mind, I'm using fish shell, so how you redirect stderr might be different in your shell

If you're piping the output to some other command, stderr isn't piped already, so there you don't need to worry anyway:

```fish
glaza watch 'live action' | xclip -r -selection clipboard
```

Since we marked that we downloaded 6 episodes, we can go and download more, by opening the 7th episode's link:

```fish
glaza save 'live action' --web
```

Sometimes urls _won't_ conveniently end in the episode number, in which case `watch` and `show` will not be available.

> well, they will be _available_, they'll just return gibberish links

In which case you should use `plink` / `pdlink` — these subcommands just print / open in browser the links literally, as provided to the `--link`/`-l` / `--dlink`/`-d` flags, without doing any magic.

```fish
glaza plink 'live action' --web
glaza pdlink 'live action' --web
```

If you want to later change the streaming link, use `link`:

```fish
glaza -g link 'live action' 'https://my_new_link.com/show_title'
```

To change the download link, use `dlink`:

```fish
glaza -g dlink 'live action' 'https://my_new_link.com/show_title'
```

Alright, let's say we finished watching the entire series, and now want to put it in our watched list.

```fish
glaza -g finish 'live action'
```

That's it! Now in our watched list, we get a line that looks something like this:

```
23.09.03 - One Piece Live Action
```

> the date format is year.month.day

If we used the `drop` subcommand instead:

```fish
glaza -g drop 'live action'
```

We would get this:

```
23.09.03 - (dropped) One Piece Live Action
```

That's basically the entire difference! For the shows that you don't want to finish, you can use `drop` to still put them in your watched list, but mark that you didn't actually _finish_ them. Also, the commit message will say `drop` instead of `finish`.

## Usage

```
`glaza` is a program for tracking the shows you watch.

All of the information is kept in three files in your data_directory/glaza, which is:
linux — $XDG_DATA_HOME/glaza
macos — $HOME/Library/Application Support/glaza
windows — %APPDATA%/glaza

`current.yml` contains the information about shows you're currently in the process of watching.
Every show has multiple properties you can set.
1. Show title
2. The episode number of the latest episode you finished watching
3. The episode number of the latest episode you *downloaded*
4. The link to where you watch (stream) the show
5. The link to where you *download* the show

Both the episode and link are separated for streaming / downloading this way, so that you can track both of those
separately. This is mostly useful with the `watch` and `save` subcommands, that let you open the next episode to
either watch, or download.

Only the show title is required, every other property is optional.
For example: if you don't need the link functionality, you can just use glaza to track the episodes.
Additionally, whenever a download link is expected but you don't have one set, the streaming link will be used as a
fallback. The opposite is true as well.
So you only really need to set both if you plan to both stream and download a show, on different websites.

`watched.txt` contains a list of all the shows you finished watching, or dropped, with timestamps. New shows appear in
this list thanks to `finish` and `drop` subcommands.

`watch-later.txt` contains a list of shows you plan to watch. You can use `add` to add new shows, and `discard` to
remove shows from that list. The `wl` will just print the contents of this file, letting you view what shows you plan
to watch.

A lot of subcommands take `show` as an argument.
`watch`, `save`, `plink`, `pdlink`, `remove`, `episode`, `save`, `link`, `discard` — all assume that the show already
exists.
The show argument is a pattern that is searched for in the show titles of current.yml (or watch-later.txt, if you use
the --grab flag in one of: `start`, `finish`, `drop`) in this order:
1. Case sensitive exact match.
2. Case insensitive exact match.
3. Case insensitive substring match.
4. Case sensitive substring match.
If after these steps no show matches, or too many shows match (more than one) — an error is printed, instead of doing
anything.

`glaza` supports git with the `--git` flag. Whenever you execute a subcommand that writes to one of the data files, if
you use the `--git` flag, a commit will be automatically created for you.
The commit messages are different per subcommand, and usually use the show title + extra information to make the git
log more useful and searchable.
The data directory is automatically `git init`ed if it isn't already.
This functionality is most useful for backing up your tracking and sharing it by uploading it to a git remote.

Usage: glaza [OPTIONS] <COMMAND>

Commands:
  shows     List all the shows you're currently watching, with their episode information [aliases: s]
  watch     Print the next episode's link. This works by appending the watched episode numer + 1 onto the link.
                This won't work if a number appended on the link doesn't result in that episode's url [aliases: next,
                go, w]
  save      Print the next download link. Works the same as the `watch` subcommand, except the `saved` episode is
                appended instead [aliases: install, i]
  plink     Print the episode link of a show. This is most useful for shows that don't support `watch` due to
                having non-standard urls
  pdlink    Print the download link of a show. This is most useful for shows that don't support `save` due to
                having non-standard urls
  episode   Set the episode you just watched [aliases: ep]
  download  Set the episode you just downloaded [aliases: dn]
  link      Update the episode link of a show. It will be used for the `watch` and `plink` subcommands. And also,
                as a fallback if you don't define a download link [aliases: ln]
  dlink     Update the download link of a show. It will be used for the `save` and `pdlink` subcommands. And also,
                as a fallback if you don't define an episode link
  start     Start a new show, putting it in your ‘currently watching’ list [aliases: new, n]
  finish    Finish a show, putting it in your watched list [aliases: f]
  drop      Drop a show, putting it in your watched list. The distinction from `finish` is that to the left of the
                show name in your watched list, there will be the `(dropped)` specifier. Also with the `--git` flag,
                the commit message will say "drop" instead of "finish" [aliases: d]
  remove    Remove a show from the list without putting it in your watched list. This is useful if you
                accidentally added a show you didn't mean to, possibly due to misspelling its title [aliases: rm,
                delete]
  add       Add a new show to your watch later list [aliases: later, a]
  discard   Remove a show from your watch later list [aliases: c]
  wl        Print the entire contents of your watch later file
  watched   Print the entire contents of your watched list [aliases: past]
  help      Print this message or the help of the given subcommand(s)

Options:
  -g, --git      If the action writes to a file, commit that change
  -h, --help     Print help
  -V, --version  Print version
```

You can also call `--help` on every individual subcommand, to inspect it further:

```fish
glaza watch --help
```

```
Print the next episode's link. This works by appending the watched episode numer + 1 onto the link. This won't work if
a number appended on the link doesn't result in that episode's url

Usage: glaza watch [OPTIONS] <SHOW>

Arguments:
  <SHOW>

Options:
  -w, --web   Open the link in your $BROWSER instead of printing it
  -h, --help  Print help
```

## Install

```
cargo install glaza
```

`cargo-binstall` and `cargo-quickinstall` are also supported.

## Uninstall

```
cargo uninstall glaza
rm -fr ~/.local/share/glaza
```
