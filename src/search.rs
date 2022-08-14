use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    thread,
};

use crate::config::SearchConfig;
pub struct SearchResult {
    pub results: Vec<SearchItemResult>,
    pub errors: Vec<String>,
}

pub struct SearchItemResult {
    pub path: String,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

pub fn search(files: Vec<PathBuf>, search_options: SearchConfig) -> SearchResult {
    let threads = files.into_iter().map(|path| {
        let query = search_options.query.to_string();
        thread::spawn(move || search_file(&path, &query, search_options.preview))
    });

    let mut search_results = Vec::new();
    let mut errors = Vec::new();
    for thread in threads {
        let result = thread.join().unwrap();
        match result {
            Ok(x) => {
                for item in x {
                    search_results.push(item);
                }
            }
            Err(x) => errors.push(x),
        }
    }

    SearchResult {
        results: search_results,
        errors: Vec::new(),
    }
}

fn search_file(
    path: &PathBuf,
    query: &String,
    preview: Option<usize>,
) -> Result<Vec<SearchItemResult>, String> {
    let mut result = Vec::new();
    let file = File::open(&path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader
                .lines()
                .flatten()
                .enumerate()
                .for_each(|(line, content)| {
                    if content.contains(query) {
                        if let Some(x) = path.to_str() {
                            let start_index = get_start_index(&content, query);
                            if let Some(si) = start_index {
                                let content = get_content_preview(&content, preview, si, query);

                                result.push(SearchItemResult {
                                    path: x.into(),
                                    content: content.trim().to_string(),
                                    line: line + 1,
                                    column: si + 1,
                                });
                            }
                        }
                    }
                });

            Ok(result)
        }
        Err(err) => Err(format!(
            "Could not read file {}: {}",
            path.to_str().unwrap_or(""),
            err
        )),
    }
}

fn get_content_preview<'a>(
    content: &'a String,
    preview: Option<usize>,
    si: usize,
    query: &String,
) -> &'a str {
    if let Some(length) = preview {
        if si > length && si + query.len() + length < content.len() {
            return &content[si - length..si + query.len() + length];
        } else if si > length {
            return &content[si - length..];
        } else if si + query.len() + length < content.len() {
            return &content[..si + query.len() + length];
        }
    }

    content
}

fn get_start_index(content: &str, query: &str) -> Option<usize> {
    let query: Vec<char> = query.chars().collect();
    let length = query.len();
    let mut temp = 0;

    for (index, char) in content.chars().enumerate() {
        if char == query[temp] {
            if temp < length - 1 {
                temp += 1;
            } else {
                return Some(index + 1 - length);
            }
        } else {
            temp = 0;
        }
    }

    None
}
