use rustgrep::{get_builder_from_config, search, Config};
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

    let files = get_builder_from_config(config.file_config).get_files();

    println!("Start search!");

    let matches = search(files, config.search_config)?;
    for m in matches {
        println!("{} {}:{}\t{}", m.path, m.line, m.column, m.content)
    }

    Ok(())
}
