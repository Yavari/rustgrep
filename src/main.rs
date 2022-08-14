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

    let file_result = get_builder_from_config(config.file_config).get_files();

    println!("Start search!");

    let results = search(file_result.files, config.search_config);
    for m in results.results {
        println!("{} {}:{}\t{}", m.path, m.line, m.column, m.content)
    }

    if !file_result.errors.is_empty() {
        eprintln!("Some errors:");
        for m in file_result.errors {
            eprintln!("{}", m)
        }
    }

    if !results.errors.is_empty() {
        eprintln!("Some errors:");
        for m in results.errors {
            eprintln!("{}", m)
        }
    }


    Ok(())
}
