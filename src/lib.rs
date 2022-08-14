mod config;
mod get_files;
mod search;

pub use config::Config;
pub use get_files::PathBuilder;
pub use search::search;

pub fn get_builder_from_config(
    path: String,
    exclude_paths: Vec<String>,
    file_types: Vec<String>,
) -> PathBuilder {
    let mut builder = get_files::path(path);
    for item in exclude_paths {
        builder = builder.exclude_folder(item);
    }

    for item in file_types {
        builder = builder.filter_file_type(item);
    }

    builder
}
