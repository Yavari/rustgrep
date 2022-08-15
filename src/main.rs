#![warn(clippy::pedantic)]
use rustgrep::{get_builder_from_config, search, Config, SearchItemResult};
use std::{env, sync::mpsc::{Sender, Receiver, channel}, thread};

fn main() {
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

    let (tx, rx): (Sender<Vec<SearchItemResult>>, Receiver<Vec<SearchItemResult>>) = channel();
    let (error_tx, error_rx): (Sender<String>, Receiver<String>) = channel();
    thread::spawn(|| {
        for file in rx {
            for item in file {
                println!(
                    "{} {}:{}\t{}",
                    item.path, item.line, item.column, item.content
                );
            }
        }
    });

    thread::spawn(|| {
        for item in error_rx {
            eprintln!("ERROR! {}", item);
        }
    });

    search(file_result.files, config.search_config, tx, error_tx);

    if !file_result.errors.is_empty() {
        eprintln!("Some errors:");
        for m in file_result.errors {
            eprintln!("{}", m);
        }
    }
}
