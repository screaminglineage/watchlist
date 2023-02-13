use clap::{ArgGroup, Args, Parser, Subcommand};
use wlist::{WatchList, WatchListError, WatchListFuncs};

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Create and Manage Watch Lists")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create new Lists
    #[clap(visible_alias = "n")]
    New(New),

    /// Add new Items
    #[clap(visible_alias = "a")]
    Add(Add),

    /// Display Lists/Items
    #[clap(visible_alias = "s")]
    Show(Show),

    /// Get a Random Item
    #[clap(visible_aliases = ["r", "rand"])]
    Random(Random),

    /// Delete Lists/Items
    #[clap(visible_aliases = ["d", "del"])]
    Delete(Delete),

    /// Searches for Items in a list
    #[clap(visible_aliases = ["se"])]
    Search(Search),
}

#[derive(Args, Debug)]
struct New {
    /// Title of new list
    pub list: String,
}

#[derive(Args, Debug)]
struct Add {
    /// List Title
    pub list: String,
    /// Items to be added
    ///
    /// Multiple items can be specified as space separated values
    #[clap(required = true)]
    pub items: Vec<String>,

    /// Ignore additions of duplicate
    /// items to the same list
    #[clap(long, short)]
    pub ignore_duplicate: bool,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("show").args(["list", "all_items"])))]
struct Show {
    /// List from which to display items
    pub list: Option<String>,

    /// Show all items from all lists
    /// excluding empty lists
    #[clap(long, short)]
    pub all_items: bool,
}

#[derive(Args, Debug)]
struct Random {
    // List from which to get random item
    pub name: Option<String>,
}

#[derive(Args, Debug)]
struct Delete {
    /// List to delete
    pub list: String,
    /// Search prompt for deletion
    ///
    /// Gives a list of items from the list which match the prompt
    pub prompt: Option<String>,
}

#[derive(Args, Debug)]
struct Search {
    /// List to search
    pub list: String,
    /// Search prompt
    pub prompt: String,
}

fn cli_delete(watchlists: &mut WatchList, delete: &Delete) -> Result<(), WatchListError> {
    if let Some(i) = &delete.prompt {
        let results = watchlists.list_search(&delete.list, i)?;
        if results.is_empty() {
            println!("No Matches");
            return Ok(());
        }
        wlist::list_display(&results, "Matched Items");

        // Validate input
        let index: usize = loop {
            let input = wlist::input("Enter Item to Delete (DEFAULT: 1): ", true)?;
            // Default option
            if input.is_empty() {
                break 1;
            }
            if let Ok(n) = input.parse::<usize>() {
                if 0 < n && n <= results.len() {
                    break n;
                }
            }
        };

        // Converts from 1-indexed list back to 0-indexed list
        watchlists.item_remove(&delete.list, &results[index - 1].to_string())?;
        println!("Item Deleted");
        return Ok(());
    }

    match wlist::input(
        &format!(
            "Are you sure you want to delete the list '{}'? (y/N): ",
            &delete.list
        ),
        true,
    )?
    .to_lowercase()
    .as_str()
    {
        "y" => {
            watchlists.list_remove(&delete.list)?;
            println!("Deleted List '{}'", &delete.list);
        }
        _ => println!("Deleting Cancelled"),
    }
    Ok(())
}

pub fn cli_run(watchlists: &mut WatchList) -> Result<(), WatchListError> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::New(new) => {
            watchlists.list_add(&new.list)?;
            println!("List Created!");
        }
        Commands::Add(add) => {
            for item in &add.items {
                watchlists.item_add(&add.list, item, add.ignore_duplicate)?
            }
            println!("Item(s) Added!");
        }
        Commands::Show(show) => {
            let all_lists = watchlists.list_get_all()?;
            if show.all_items {
                for list in all_lists {
                    // Index into watchlists cannot fail as the list_get_all method
                    // already returns all the keys of watchlists
                    // TODO: figure out a better way to do this though
                    let items = &watchlists[list];
                    if !items.is_empty() {
                        wlist::list_display(items, list);
                        println!();
                    }
                }
            // Display List Items
            } else if let Some(l) = &show.list {
                let items = watchlists.item_get_all(l)?;
                wlist::list_display(items, l);
            // Display All List Titles
            } else {
                wlist::list_display(&all_lists, "All Lists");
            }
        }
        Commands::Random(random) => {
            if let Some(n) = &random.name {
                println!("{}", watchlists.item_get_random(n)?);
            } else {
                println!(
                    "{}",
                    loop {
                        let list = watchlists.list_get_random()?;
                        // list will always be a key of watchlists
                        // due to the list_get_random function
                        if !watchlists[list].is_empty() {
                            break watchlists.item_get_random(list)?;
                        }
                    }
                );
            }
        }
        Commands::Delete(delete) => cli_delete(watchlists, delete)?,
        Commands::Search(search) => {
            let results = watchlists.list_search(&search.list, &search.prompt)?;
            if results.is_empty() {
                println!("No Matches");
                return Ok(());
            }
            wlist::list_display(&results, "Matches");
        }
    }

    Ok(())
}
