# glaza

> ‘glaza’ means ‘eyes’ in russian. It's also what you use to watch shows!

Track the shows you watch using your terminal!

`glaza` will create a directory where it will store its files.

That directory is `~/.local/share/glaza` on linux and `%AppData%\glaza` on windows.

Every time you specify the `-g`/`--git` flag, the action you do will be commited, if it writes to one of the files.

If you never specify the `--git` flag, the data directory won't be git initialized.

Example usage:
```
glaza -g show set ep -s "One Piece Live Action" -e 1
glaza show set ep -s "One Piece Live Action" -e 2
glaza --git show set ep -s "One Piece Live Action" -e 3
glaza -g show finish -s "One Piece Live Action"
```

You get to decide which actions are big enough for you to commit, and which ones are not.

```
750cd69 (HEAD -> main, origin/main, origin/HEAD) finish One Piece Live Action
1872abc watch ep3 -> One Piece Live Action
275dafb watch ep1 -> One Piece Live Action
```

glaza is basically an abstraction to interact with three files:

### 1. current.json

Contains the shows you're currently watching.

Each show has three properties:
1. link
2. episode
3. downloaded

Some shows you just stream, others you may download: that's why there's a downloaded property for you to be able to easily track the difference of episodes you've *watched* and episodes you've only downloaded.

You're supposed to set both the episode and downloaded properties to the episode you *finished*. That way, you'll be able to use the `show watch` and `show download` subcommands to get the link to the *next* episode you want to watch / download.

Both of those subcommands work by getting the link in the link property and concatenating it with the episode / downloaded property + 1, resulting in the link to the next episode. So if your show link ends in an episode number, **remove** that part of the link when you `show new` or `show set link`

By default, that link is printed to stdout. But if you provide the `--open`/`-o` flag, it will get opened in your $BROWSER

Once you finish or drop a show, you can execute `show finish` (or `show drop`) to move it to your watched list, removing it from your current.json

### 2. watched.txt

This file contains all the shows you've ever finished or dropped.

The difference between the two is that the latter is expressed in a tag that looks like: `(dropped)`

Whenever a show gets added here, it also contains the date of finishing at the start of the line.

So it's a good idea to mark a show as finished as soon as you actually do, to reflect the date correctly! :)

To view all of your past watched shows, run `show past`

If you added a show by mistake, and don't intend to actually watch it, you can use `show remove` to remove a show from your current.json without putting it in your watched list

### 3. watch-later.txt

A simple "watch later" file, that you can `list`, `add` or `remove` from.

The cool thing about it is the abstraction subcommand of `wl start`.

It lets you pick a show from your watch later list, remove it from there and create a new show in your current.json.

Yes, this:
```
glaza wl start -s "Show Name: The return of the beast season 2 reimagined" -l "https://yourOptionalLink.com"
```

Is eqivalent to running these two commands:
```
glaza wl remove -s "Show Name: The return of the beast season 2 reimagined"
glaza show new -s "Show Name: The return of the beast season 2 reimagined" -l "https://yourOptionalLink.com"
```

Quite a long title isn't it? Having both actions in a single subcommand is gonna save you some time.

The link flag is optional in creating shows, because maybe you don't even have a link to the show (maybe you don't pirate 🤯) and just want to use the program to track the episodes.

## Usage

```
A program to help you track shows you're watching

Usage: glaza [OPTIONS] <COMMAND>

Commands:
  show  Commands to interact with the shows you're currently watching or have watched
  wl    Commands to interact with your "Watch later" list

Options:
  -g, --git      If the action writes to a file, commit that change
  -h, --help     Print help
  -V, --version  Print version
```
```
Usage: glaza show <COMMAND>

Commands:
  watch     Print or open the next episode's link
  download  Print or open the next download link [aliases: dn]
  link      Print or open the link of a show
  finish    Finish a show, putting it in your watched list
  drop      Drop a show, putting it in your watched list
  new       Start a new show
  list      List all the shows you're currently watching
  past      Print the entirety of the watched list
  remove    Remove a show from the list without putting it in your watched list [aliases: rm]
  set       Update a show's properties
```
```
Usage: glaza show set <COMMAND>

Commands:
  episode   Set the episode you just watched [aliases: ep]
  download  Set the episode you just downloaded [aliases: dn]
  link      Set the link
```
```
Usage: glaza wl <COMMAND>

Commands:
  add     Add a new show to your watch later list
  remove  Remove a show from your watch later list
  start   Remove a show from your watch later list, and start watching it
  list    Print the entire contents of your watch later file
```

You can use `-h`/`--help` anywhere! If you want to learn about the flags of a specific subcommand, run something like `glaza show watch --help`

## Examples

I watched the Baki anime, so I start not at chapter 1 of the manga. I make use of the `--git` flag to only make one commit for adding it.
```
glaza show new -s "Baki Dou" -l "this would be the link"
glaza --git show set ep -s "Baki Dou" -e 69
```
Adding a name this long isn't that big of a deal because of shell history.
```
glaza --git wl add -s 'Richard Hammack — Book of Proof'
```
The website I set initially isn't as good as another one I found, so I updated the link.
```
glaza --git show set link -s "One Piece Manga" -l "this is once again a link"
```
Good movie.
```
glaza --git show finish -s 'Lord of War'
```

## Install

```
cargo install glaza
```

`cargo-binstall` and `cargo-quickinstall` are also supported

## Uninstall

```
cargo uninstall glaza
rm -fr ~/.local/share/glaza
```
