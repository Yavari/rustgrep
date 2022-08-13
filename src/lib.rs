mod get_files;
mod search;
mod config;

pub use get_files::PathBuilder;
pub use search::search;
pub use config::Config;

pub fn get_builder_from_config(path: String, exclude_paths: Vec<String>) -> PathBuilder {
    let mut builder = get_files::path(path);
    for item in exclude_paths {
        builder = builder.exclude_folder(item);
    }

    builder
}