use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    fs::{self},
    path::PathBuf,
    thread,
};

pub fn path(path: String) -> PathBuilder {
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

    pub fn get_files(self) -> Vec<PathBuf> {
        get_files_inner(&self.path, &self.exclude_paths)
    }
}

fn get_files_inner(path: &OsString, exclude_paths: &HashMap<OsString, bool>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let result = fs::read_dir(path);
    if let Ok(result) = result {
        let result = result.filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .filter(|x| !x.path().is_dir() || exclude_paths.get(&x.file_name()) == None);

        let mut threads = Vec::new();
        for item in result {
            if !item.path().is_dir() {
                files.push(item.path());
            } else {
                let folder_path = create_path(path, &item.file_name());
                let a = exclude_paths.clone();
                let handle = thread::spawn(move || {
                    get_files_inner(&folder_path, &a)
                });

                threads.push(handle);
            }
        }

        for handle in threads {
            let result = handle.join().unwrap();
            for item in result {
                files.push(item);
            }
        }
    }

    files
}

fn create_path(prefix: &OsStr, folder: &OsStr) -> OsString {
    let mut path: OsString = prefix.into();
    path.push(folder);
    path.push("/");
    path
}
