use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::{error, collections::HashMap};
use std::fs;

mod search_pattern;
pub use search_pattern::path;

pub struct Item {
    pub file_name: std::ffi::OsString,
    pub path: PathBuf,
}

pub fn search(search_token: SearchToken) -> Result<Vec<Item>, Box<dyn error::Error>> {
    search_inner(&search_token.path, &search_token.exclude_paths)
}

fn search_inner(path: &OsString, exclude_paths: &HashMap<OsString, bool>) -> Result<Vec<Item>, Box<dyn error::Error>> {
    let result = fs::read_dir(path)?
    .filter(|x| x.is_ok())
    .map(|x| x.unwrap())
    .map(|x| Item {
        file_name: x.file_name(), 
        path: x.path()
    })
    .filter(|x| !x.path.is_dir() || exclude_paths.get(&x.file_name) == None);

    let mut items: Vec<Item> = Vec::new();
    for item in result {
        if !item.path.is_dir() {
            items.push(item);
        } else {
            let folder_path = create_path(path, &item.file_name);
            items.push(item);
            items.append(&mut search_inner(&folder_path, exclude_paths)?);
        }
    }

    Ok(items)
}

fn create_path(prefix: &OsStr, folder: &OsStr) -> OsString {
    let mut path:OsString = prefix.into();
    path.push(folder);
    path.push("/");
    path
}

pub struct SearchToken {
    path: OsString,
    exclude_paths: HashMap<OsString, bool>
}