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
        file_types: None,
    }
}

pub struct FileResult {
    pub files: Vec<PathBuf>,
    pub errors: Vec<String>,
}

#[must_use]
pub struct PathBuilder {
    path: OsString,
    exclude_paths: HashMap<OsString, bool>,
    file_types: Option<HashMap<String, bool>>,
}

impl PathBuilder {
    pub fn exclude_folder(mut self, exclude_path: String) -> PathBuilder {
        self.exclude_paths.insert(exclude_path.into(), true);
        self
    }

    pub fn filter_file_type(mut self, file_type: String) -> PathBuilder {
        if self.file_types.is_none() {
            self.file_types = Some(HashMap::new());
        }

        if let Some(ref mut file_types) = self.file_types {
            file_types.insert(file_type, true);
        }

        self
    }

    #[must_use]
    pub fn get_files(self) -> FileResult {
        let (files, errors) = get_files_inner(&self.path, &self.exclude_paths, &self.file_types);
        FileResult { files, errors }
    }
}

fn get_files_inner(
    path: &OsString,
    exclude_paths: &HashMap<OsString, bool>,
    file_types: &Option<HashMap<String, bool>>,
) -> (Vec<PathBuf>, Vec<String>) {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let result = fs::read_dir(path);
    match result {
        Ok(result) => {
            let result = result
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .filter(|x| !x.path().is_dir() || exclude_paths.get(&x.file_name()) == None);

            let mut threads = Vec::new();
            for item in result {
                if item.path().is_dir() {
                    let folder_path = create_path(path, &item.file_name());
                    let exclude_paths = exclude_paths.clone();
                    let file_types = file_types.clone();
                    let handle = thread::spawn(move || {
                        get_files_inner(&folder_path, &exclude_paths, &file_types)
                    });

                    threads.push(handle);
                } else {
                    match file_types {
                        Some(file_types) => {
                            if let Some(filename) = item.file_name().to_str() {
                                if let Some(file_extension) = filename.split('.').last() {
                                    if file_types.contains_key(file_extension) {
                                        files.push(item.path());
                                    }
                                }
                            }
                        }
                        None => files.push(item.path()),
                    }
                }
            }

            for handle in threads {
                let result = handle.join().unwrap();
                for item in result.0 {
                    files.push(item);
                }

                for item in result.1 {
                    errors.push(item);
                }
            }
        }
        Err(e) => errors.push(e.to_string()),
    }

    (files, errors)
}

fn create_path(prefix: &OsStr, folder: &OsStr) -> OsString {
    let mut path: OsString = prefix.into();
    path.push(folder);
    path.push("/");
    path
}
