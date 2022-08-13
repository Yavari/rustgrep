use rustgrep::search;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let query = "self".to_string();
    let files = rustgrep::path("./")
        .exclude_folder(".git".to_string())
        .exclude_folder("target".to_string())
        .exclude_folder(".vscode".to_string())
        .get_files()?;

    let matches = search(files, query)?;
    for m in matches {
        println!("{} {}:{}\t{}", m.path, m.line, m.column, m.content)
    }

    Ok(())
}
