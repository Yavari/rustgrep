use std::{
    error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    thread,
};

pub struct SearchResult {
    pub path: String,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

pub fn search(
    files: Vec<PathBuf>,
    query: String,
) -> Result<Vec<SearchResult>, Box<dyn error::Error>> {
    let threads = files.into_iter().map(|path| {
        let a = query.to_string();
        thread::spawn(move || {
            let mut result = Vec::new();
            let file = File::open(&path);
            if let Ok(file) = file {
                let reader = BufReader::new(file);
                reader
                    .lines()
                    .flatten()
                    .enumerate()
                    .for_each(|(line, content)| {
                        if content.contains(&a) {
                            if let Some(x) = path.to_str() {
                                let start_index = get_start_index(&content, &a);
                                println!("{}", x);
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

            result
        })
    });

    let mut search_results = Vec::new();
    for thread in threads {
        let result = thread.join().unwrap();
        for item in result {
            search_results.push(item);
        }
    }

    Ok(search_results)
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
