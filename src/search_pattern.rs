use std::{ffi::OsString, collections::HashMap};

use crate::SearchToken;

pub fn path(path: &str) -> Pattern {
    Pattern { 
        path: path.into(),
        exclude_paths: HashMap::new()
    }
}

pub struct Pattern {
    path: OsString,
    exclude_paths: HashMap<OsString, bool>
}

impl Pattern {
    pub fn exclude_folder(mut self, exclude_path: String) -> Pattern {
        self.exclude_paths.insert(exclude_path.into(), true);
        self
    }

    pub fn build(self) -> SearchToken {
        SearchToken { 
            path:self.path,
            exclude_paths: self.exclude_paths
        }
    }
}