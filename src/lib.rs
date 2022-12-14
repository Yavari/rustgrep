//#![warn(clippy::pedantic)]
mod config;
mod get_files;
mod search;

pub use config::Config;
use config::FileConfig;
pub use get_files::PathBuilder;
pub use search::search;
pub use search::ItemResult;

pub fn get_builder_from_config(file_config: FileConfig) -> PathBuilder {
    let mut builder = get_files::path(file_config.path);
    for item in file_config.exclude_paths {
        builder = builder.exclude_folder(item);
    }

    if let Some(file_types) = file_config.file_types {
        for item in file_types {
            builder = builder.filter_file_type(item);
        }
    }

    builder
}
