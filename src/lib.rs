use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{read_to_string, File};
use std::io::{self, Write};
use std::path::Path;

use colored::Colorize;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum WatchListError {
    NoTitles,
    EmptyList(String),
    TitleAlreadyPresent(String),
    TitleNotPresent(String),
    ItemAlreadyPresent(String, String),
    ItemToRemoveNotPresent(String),
    IOError(io::Error),
}

// Rename to WatchList and delete above struct when done
pub type WatchList = HashMap<String, Vec<String>>;

pub trait WatchListFuncs<'a> {
    fn from_file(file_path: &Path) -> io::Result<Self>
    where
        Self: Sized;
    fn to_file(&self, file_path: &Path) -> io::Result<()>;
    fn item_add(
        &mut self,
        title: &str,
        item: &str,
        no_duplicate: bool,
    ) -> Result<(), WatchListError>;
    fn item_remove(&mut self, title: &str, item: &str) -> Result<(), WatchListError>;
    fn item_get_all(&self, title: &str) -> Result<&Vec<String>, WatchListError>;
    fn item_get_random(&'a self, title: &str) -> Result<&'a String, WatchListError>;
    fn list_add(&mut self, title: &str) -> Result<(), WatchListError>;
    fn list_remove(&mut self, title: &str) -> Result<(), WatchListError>;
    fn list_get_all(&self) -> Result<Vec<&String>, WatchListError>;
    fn list_get_random(&'a self) -> Result<&'a String, WatchListError>;
    fn list_search(&self, title: &str, search: &str) -> Result<Vec<&String>, WatchListError>;
}

impl<'a> WatchListFuncs<'a> for WatchList {
    fn from_file(file_path: &Path) -> io::Result<Self> {
        let data = read_to_string(file_path)?;
        let watchlist: WatchList = serde_json::from_str(&data)?;
        Ok(watchlist)
    }

    fn to_file(&self, file_path: &Path) -> io::Result<()> {
        let file = File::create(file_path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    fn item_add(
        &mut self,
        title: &str,
        item: &str,
        add_duplicate: bool,
    ) -> Result<(), WatchListError> {
        let list_items = self
            .get_mut(title)
            .ok_or(WatchListError::TitleNotPresent(title.to_string()))?;

        // Ignoring duplicate items if specified
        if add_duplicate {
            list_items.push(item.to_string());
            return Ok(());
        }

        // Check for duplicate items before adding otherwise
        match list_items.iter().position(|l| l == item) {
            Some(_) => Err(WatchListError::ItemAlreadyPresent(
                item.to_string(),
                title.to_string(),
            ))?,
            None => {
                list_items.push(item.to_string());
                Ok(())
            }
        }
    }

    fn item_remove(&mut self, title: &str, item: &str) -> Result<(), WatchListError> {
        let list_items = self
            .get_mut(title)
            .ok_or(WatchListError::TitleNotPresent(title.to_string()))?;
        let index = list_items
            .iter()
            .position(|l| l == item)
            .ok_or(WatchListError::ItemToRemoveNotPresent(item.to_string()))?;
        list_items.remove(index);

        Ok(())
    }

    fn item_get_all(&self, title: &str) -> Result<&Vec<String>, WatchListError> {
        let items = self
            .get(title)
            .ok_or(WatchListError::TitleNotPresent(title.to_string()))?;
        if items.is_empty() {
            return Err(WatchListError::EmptyList(title.to_string()));
        }
        Ok(items)
    }

    fn item_get_random(&'a self, title: &str) -> Result<&'a String, WatchListError> {
        let mut rng = rand::thread_rng();
        let random_item = self
            .get(title)
            .ok_or(WatchListError::TitleNotPresent(title.to_string()))?
            .choose(&mut rng)
            .ok_or(WatchListError::EmptyList(title.to_string()))?;
        Ok(random_item)
    }

    fn list_add(&mut self, title: &str) -> Result<(), WatchListError> {
        match self.contains_key(title) {
            true => Err(WatchListError::TitleAlreadyPresent(title.to_string())),
            false => {
                self.insert(title.to_string(), Vec::new());
                Ok(())
            }
        }
    }

    fn list_remove(&mut self, title: &str) -> Result<(), WatchListError> {
        self.remove(title)
            .ok_or(WatchListError::TitleNotPresent(title.to_string()))?;
        Ok(())
    }

    fn list_get_all(&self) -> Result<Vec<&String>, WatchListError> {
        let list_titles: Vec<&String> = self.keys().collect();
        if list_titles.is_empty() {
            return Err(WatchListError::NoTitles);
        }
        Ok(list_titles)
    }

    fn list_get_random(&'a self) -> Result<&'a String, WatchListError> {
        let mut rng = rand::thread_rng();
        let lists = self.list_get_all()?;
        let random_list = lists.choose(&mut rng).ok_or(WatchListError::NoTitles)?;
        Ok(random_list)
    }

    fn list_search(&self, title: &str, search: &str) -> Result<Vec<&String>, WatchListError> {
        Ok(self
            .get(title)
            .ok_or(WatchListError::TitleNotPresent(title.to_string()))?
            .iter()
            .filter(|i| i.to_ascii_lowercase().contains(&search.to_lowercase()))
            .collect())
    }
}

pub fn input(prompt: &str, trim_input: bool) -> Result<String, WatchListError> {
    let mut input = String::new();
    print!("{prompt}");
    io::stdout()
        .flush()
        .map_err(WatchListError::IOError)?;
    io::stdin()
        .read_line(&mut input)
        .map_err(WatchListError::IOError)?;

    if trim_input {
        return Ok(input.trim().to_string());
    }
    Ok(input)
}

pub fn list_display<T>(list: &[T], title: &str)
where
    T: Display + Sized,
{
    println!("{: ^15}", title.italic().underline());

    for (i, item) in list.iter().enumerate() {
        println!("{: >5}. | {: <10}", format!("{}", i + 1).bold(), item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> WatchList {
        let data = r#"
        {
            "Movies": [ 
                "Movie 1", 
                "Movie 2", 
                "Movie 3", 
                "Movie 4" 
            ],

            "Manga": [ 
                "Manga 1", 
                "Manga 2", 
                "Manga 3", 
                "Manga 4" 
            ]

        }"#;
        serde_json::from_str(data).unwrap()
    }

    #[test]
    fn add() {
        let mut watchlist = setup();
        watchlist.item_add("Movies", "Movie 5", false).unwrap();
        watchlist.item_add("Movies", "Movie 10", false).unwrap();
        watchlist.item_add("Manga", "Manga 100", false).unwrap();
        assert_eq!(
            watchlist["Movies"],
            vec!["Movie 1", "Movie 2", "Movie 3", "Movie 4", "Movie 5", "Movie 10"]
        );

        assert_eq!(
            watchlist["Manga"],
            vec!["Manga 1", "Manga 2", "Manga 3", "Manga 4", "Manga 100"]
        );
    }

    #[test]
    fn add_errors() {
        let mut watchlist = setup();
        assert_eq!(
            watchlist.item_add("TV", "Manga 999", false).err().unwrap(),
            WatchListError::TitleNotPresent("TV".to_string())
        );

        assert_eq!(
            watchlist
                .item_add("Movies", "Movie 1", false)
                .err()
                .unwrap(),
            WatchListError::ItemAlreadyPresent("Movie 1".to_string(), "Movies".to_string())
        );
    }

    #[test]
    fn remove() {
        let mut watchlist = setup();
        watchlist.item_remove("Movies", "Movie 3").unwrap();
        watchlist.item_remove("Manga", "Manga 3").unwrap();
        watchlist.item_remove("Manga", "Manga 1").unwrap();
        assert_eq!(watchlist["Movies"], vec!["Movie 1", "Movie 2", "Movie 4"]);
        assert_eq!(watchlist["Manga"], vec!["Manga 2", "Manga 4"]);
    }

    #[test]
    fn remove_errors() {
        let mut watchlist = setup();
        assert_eq!(
            watchlist.item_remove("TV", "Manga 999").err().unwrap(),
            WatchListError::TitleNotPresent("TV".to_string())
        );

        assert_eq!(
            watchlist.item_remove("Movies", "Manga 999").err().unwrap(),
            WatchListError::ItemToRemoveNotPresent("Manga 999".to_string())
        );
    }

    #[test]
    fn random() {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        let mut watchlist = setup();
        let lists = vec!["Movies".to_string(), "Manga".to_string()];
        let list = lists.choose(&mut rng).unwrap();

        let item = watchlist.item_get_random(list).unwrap().clone();
        assert!(watchlist.get(list).unwrap().contains(&item));
        println!("{item} is present in {list}");
    }

    #[test]
    fn random_errors() {
        let mut watchlist = setup();

        assert_eq!(
            watchlist.item_get_random("Random!").err().unwrap(),
            WatchListError::TitleNotPresent("Random!".to_string())
        );

        watchlist.insert("NEW LIST".to_string(), vec![]);
        assert_eq!(
            watchlist.item_get_random("NEW LIST").err().unwrap(),
            WatchListError::EmptyList("NEW LIST".to_string())
        );
    }

    #[test]
    fn adding_new_title() {
        let mut watchlist = setup();
        watchlist.list_add("Anime").unwrap();
        watchlist.list_add("AMOGUS").unwrap();
        assert_eq!(watchlist["Anime"], Vec::<String>::new());
        assert_eq!(watchlist["AMOGUS"], Vec::<String>::new());
    }

    #[test]
    fn adding_new_title_errors() {
        let mut watchlist = setup();
        assert_eq!(
            watchlist.list_add("Movies").err().unwrap(),
            WatchListError::TitleAlreadyPresent("Movies".to_string())
        );
    }

    #[test]
    fn removing_title() {
        let mut watchlist = setup();
        watchlist.list_remove("Movies").unwrap();
        assert_eq!(watchlist.get("Movies"), None);
    }

    #[test]
    fn removing_title_errors() {
        let mut watchlist = setup();
        assert_eq!(
            watchlist.list_remove("TV").err().unwrap(),
            WatchListError::TitleNotPresent("TV".to_string())
        );
    }

    /*#[test]
    fn read_from_file() {
        const FILE_PATH: &'static str = "data.json";

        let watchlist = setup();
        let lists: Vec<WatchList> = (1..=10).map(|_| watchlist.clone()).collect();
        let file = File::create(FILE_PATH).expect("failed to create file");
        serde_json::to_writer(file, &lists).expect("failed to write");

        let watchlists = watchlist_read_file(Path::new(FILE_PATH)).expect("failed to read file");

        for w in watchlists {
            assert_eq!(watchlist, w);
        }
    }

    #[test]
    fn write_to_file() {
        const FILE_PATH: &'static str = "test.json";

        let watchlist = setup();
        let lists: Vec<WatchList> = (1..=10).map(|_| watchlist.clone()).collect();
        watchlist_write_file(Path::new(FILE_PATH), &lists).expect("failed to write");

        let file = File::open(FILE_PATH).expect("failed to open file");
        let new_lists: Vec<WatchList> = serde_json::from_reader(file).expect("failed to read");

        for w in new_lists {
            assert_eq!(watchlist, w);
        }
    }*/
}
