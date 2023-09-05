# floral_barrel

Track the shows you watch using your terminal!

floral_barrel will create a directory where it will store its files.

That directory is stored in `~/.local/share/floral_barrel` on linux.

You can `git init` it to make the `-g`/`--git` flag to work.

Every time you specify the `--git` flag, the action you do will be commited

Example:
```
floral_barrel -g show set ep -s "One Piece Live Action" -e 1
floral_barrel -g show set ep -s "One Piece Live Action" -e 2
floral_barrel -g show set ep -s "One Piece Live Action" -e 3
floral_barrel -g show finish -s "One Piece Live Action"
git long --oneline
```
```
750cd69 (HEAD -> main, origin/main, origin/HEAD) finish One Piece Live Action
1872abc watch ep3 -> One Piece Live Action
b83a21c watch ep2 -> One Piece Live Action
275dafb watch ep1 -> One Piece Live Action
```

floral_barrel is basically an abstraction to interact with three files:
### 1. shows.json

Contains the shows you're currently watching.

Each show has three properties:
1. link
2. episode
3. downloaded

Some shows you just stream, others you may download: that's why there's a downloaded property for you to be able to easily track the difference of episodes you've *watched* and episodes you've just downloaded.

You're supposed to set both the episode and downloaded properties to the episode you *finished*. That way, you'll be able to use the `show watch` and `show download` subcommands to get the link to the *next* episode you want to watch / download.

Both of those subcommands work by getting the link in the link property and concatenating it with the episode / downloaded property + 1, resulting in the link to the next episode.