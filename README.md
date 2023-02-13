# watchlist
A Simple CLI Tool to Add and Manage Watch Lists

The binary is named `wl` for ease of typing.
## Available Subcommands
```
Create and Manage Watch Lists

Usage: wl <COMMAND>

Commands:
  new     Create new Lists [aliases: n]
  add     Add new Items [aliases: a]
  show    Display Lists/Items [aliases: s]
  random  Get a Random Item [aliases: r, rand]
  delete  Delete Lists/Items [aliases: d, del]
  search  Searches for Items in a list [aliases: se]
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
Help for the respective subcommands can be seen using the `help` subcommand and then the name of the command, `wl help add`, or by using the `-h` option, `wl add -h`
The aliases, `n`, `a`, `s`, `r`, `d` and `se` can be used for each of the subcommands respectively.

### Create New Lists
```
Usage: wl new <LIST>

Arguments:
  <LIST>  Title of new list

Options:
  -h, --help     Print help
  -V, --version  Print version
```
### Add Items to Created Lists
```
Usage: wl add [OPTIONS] <LIST> <ITEMS>...

Arguments:
  <LIST>      List Title
  <ITEMS>...  Items to be added

Options:
  -i, --ignore-duplicate  Ignore additions of duplicate items to the same list
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version
  ```
### Display Lists
```
Usage: wl show [OPTIONS] [LIST]

Arguments:
  [LIST]  List from which to display items

Options:
  -a, --all-items  Show all items from all lists excluding empty lists
  -h, --help       Print help
  -V, --version    Print version
```
### Get a Random Item
```
Usage: wl random [NAME]

Arguments:
  [NAME]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
### Delete List/Item
```
Usage: wl delete <LIST> [PROMPT]

Arguments:
  <LIST>    List to delete
  [PROMPT]  Search prompt for deletion

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```
### Search for Item(s) in List
```
Usage: wl search <LIST> <PROMPT>

Arguments:
  <LIST>    List to search
  <PROMPT>  Search prompt

Options:
  -h, --help     Print help
  -V, --version  Print version
```
  
