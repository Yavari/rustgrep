use std::error;
use rustgrep::search;

fn main() -> Result<(), Box<dyn error::Error>> {
    let pattern = rustgrep::path("./")
    .exclude_folder(".git".to_string())
    .exclude_folder("target".to_string())
    .exclude_folder(".vscode".to_string())
    .build();

    let paths = search(pattern)?;
    for path in paths {
        println!("{:?} {:?}", path.path, path.file_name, );
    }

    Ok(())
}