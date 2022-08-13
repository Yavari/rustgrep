use std::{
    error,
    fs::{DirEntry, File},
    io::{BufRead, BufReader},
};

pub struct SearchResult {
    pub path: String,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

pub fn search(
    files: Vec<DirEntry>,
    query: String,
) -> Result<Vec<SearchResult>, Box<dyn error::Error>> {
    let mut result = Vec::new();
    for path in files {
        let file = File::open(&path.path())?;
        let reader = BufReader::new(file);
        reader
            .lines()
            .flatten()
            .enumerate()
            .for_each(|(line, content)| {
                if content.contains(&query) {
                    if let Some(x) = path.path().to_str() {
                        let start_index = get_start_index(&content, &query);
                        if let Some(si) = start_index {
                            result.push(SearchResult {
                                path: x.into(),
                                content: content.trim().to_string(),
                                line: line + 1,
                                column: si + 1,
                            });
                        }
                    }
                }
            });
    }

    Ok(result)
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
                return Some(index - length + 1);
            }
        } else {
            temp = 0;
        }
    }

    None
}
