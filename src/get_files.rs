use std::{
    collections::HashMap,
    error,
    ffi::{OsStr, OsString},
    fs::{self}, path::PathBuf,
};


pub fn path(path: &str) -> PathBuilder {
    PathBuilder {
        path: path.into(),
        exclude_paths: HashMap::new(),
    }
}

pub struct PathBuilder {
    path: OsString,
    exclude_paths: HashMap<OsString, bool>,
}

impl PathBuilder {
    pub fn exclude_folder(mut self, exclude_path: String) -> PathBuilder {
        self.exclude_paths.insert(exclude_path.into(), true);
        self
    }

    pub fn get_files(self) -> Result<Vec<PathBuf>, Box<dyn error::Error>> {
        get_files_inner(&self.path, &self.exclude_paths)
    }
}

fn get_files_inner(
    path: &OsString,
    exclude_paths: &HashMap<OsString, bool>,
) -> Result<Vec<PathBuf>, Box<dyn error::Error>> {
    let result = fs::read_dir(path)?
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| !x.path().is_dir() || exclude_paths.get(&x.file_name()) == None);

    let mut files: Vec<PathBuf> = Vec::new();
    for item in result {
        if !item.path().is_dir() {
            files.push(item.path());
        } else {
            let folder_path = create_path(path, &item.file_name());
            files.append(&mut get_files_inner(&folder_path, exclude_paths)?);
        }
    }

    Ok(files)
}

fn create_path(prefix: &OsStr, folder: &OsStr) -> OsString {
    let mut path: OsString = prefix.into();
    path.push(folder);
    path.push("/");
    path
}
