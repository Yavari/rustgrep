use rustgrep::{search, Config, get_builder_from_config};
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = match Config::build(env::args()) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            println!("Using default config.");
            Config::default()
        }
    };

    println!("{}", config);

    let files = get_builder_from_config(config.path, config.exclude_paths).get_files();

    println!("Start search!");

    let matches = search(files, config.query)?;
    for m in matches {
        println!("{} {}:{}\t{}", m.path, m.line, m.column, m.content)
    }

    Ok(())
}
