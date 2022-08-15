use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use crate::config::SearchConfig;

pub struct ItemResult {
    pub path: String,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

pub fn search(files: Vec<PathBuf>, search_options: SearchConfig, tx: &Sender<Vec<ItemResult>>, error_tx: &Sender<String>) {
    let threads: Vec<JoinHandle<()>> = files
        .into_iter()
        .map(|path| {
            let query = search_options.query.to_string();
            let tx = tx.clone();
            let error_tx = error_tx.clone();
            thread::spawn(move || search_file(&path, &query, search_options.preview, &tx, &error_tx))
        })
        .collect();

    for thread in threads {
        thread.join().unwrap_or_default();
    }
}

fn search_file(
    path: &PathBuf,
    query: &String,
    preview: Option<usize>,
    tx: &Sender<Vec<ItemResult>>,
    error_tx: &Sender<String>,
) {
    let file = File::open(&path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut items = Vec::new();
            for (line, content) in reader.lines().flatten().enumerate() {
                if content.contains(query) {
                    if let Some(x) = path.to_str() {
                        let start_index = get_start_index(&content, query);
                        if let Some(si) = start_index {
                            let content = get_content_preview(&content, preview, si, query);
                            items.push(ItemResult {
                                path: x.into(),
                                content: content.trim().to_string(),
                                line: line + 1,
                                column: si + 1,
                            });
                        }
                    }
                }
            }

            let r = tx.send(items);

            if r.is_err() {
            }
        }
        Err(err) => {
            let message = format!(
                "Could not read file {}: {}",
                path.to_str().unwrap_or(""),
                err);
            if error_tx.send(message).is_err() {
            }
        }
    };
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
