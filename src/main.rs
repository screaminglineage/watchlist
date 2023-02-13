use std::env;
use std::error;
use std::io;
use std::{collections::HashMap, path::Path};

use wlist::{WatchList, WatchListError::*, WatchListFuncs};
mod cli;

const WATCHLIST_FILE_PATH: &str = "watchlist.json";
const WATCHLIST_ENV_VAR: &str = "WATCHLIST_FILE_PATH";

fn main() -> Result<(), Box<dyn error::Error>> {
    let file_path = get_file_path();
    let mut watchlists = match WatchList::from_file(Path::new(&file_path)) {
        Ok(w) => w,
        Err(e) if e.kind() == io::ErrorKind::NotFound => HashMap::new(),

        // Handles IsADirectory Error with error code 21
        // TODO: Replace with e.kind() == Errorkind::IsADirectory once it becomes stable
        Err(e) if e.raw_os_error() == Some(21) => {
            println!("Error! Couldnt Find File!\nCheck if the environment variable is set to a file and not a directory");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e)),
    };

    match cli::cli_run(&mut watchlists) {
        Err(NoTitles) => eprintln!("No Lists Found!\nCreate a new one using the `new` subcommand. See `wl --help` for more info"),
        Err(EmptyList(t)) => eprintln!("No Items Added to List - {t}!"),
        Err(TitleNotPresent(t)) => eprintln!("No such list - {t}!"),
        Err(ItemAlreadyPresent(i, t)) => eprintln!("{i} is already in the list - {t}!"),
        Err(ItemToRemoveNotPresent(i)) => eprintln!("{i} not in the list!"),
        Err(TitleAlreadyPresent(t)) => eprintln!("A list called {t} already exists"),
        Err(IOError(e)) => eprintln!("{e}"),

        Ok(()) => {}
    }
    watchlists.to_file(Path::new(&file_path))?;
    Ok(())
}

fn get_file_path() -> String {
    match env::var(WATCHLIST_ENV_VAR) {
        Ok(path) => path,
        Err(_) => WATCHLIST_FILE_PATH.to_string(),
    }
}
