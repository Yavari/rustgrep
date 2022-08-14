use rustgrep::{get_builder_from_config, search, Config};
use std::{env, error, thread};

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

    let result = search(file_result.files, config.search_config);

    thread::spawn(|| {
        for file in result.rx {
            for item in file{
                println!(
                    "{} {}:{}\t{}",
                    item.path, item.line, item.column, item.content
                )
            }
        }
    });

    thread::spawn(|| {
        for item in result.error_rx {
            eprintln!("ERROR! {}", item)
        }
    });

    for item in result.tasks {
        item.join().unwrap();
    }

    if !file_result.errors.is_empty() {
        eprintln!("Some errors:");
        for m in file_result.errors {
            eprintln!("{}", m)
        }
    }

    Ok(())
}
