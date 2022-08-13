use rustgrep::{search, Config};
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = match Config::build(env::args()) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            println!("Using default config.");
            Config::default()
        },
    };

    let mut builder = rustgrep::path(&config.path);

    for item in config.exclude_paths {
        builder = builder.exclude_folder(item);
    }

    let files = builder.get_files()?;

    let matches = search(files, config.query)?;
    for m in matches {
        println!("{} {}:{}\t{}", m.path, m.line, m.column, m.content)
    }

    Ok(())
}
