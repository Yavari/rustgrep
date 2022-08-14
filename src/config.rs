use std::fmt;

pub struct Config {
    pub search_config: SearchConfig,
    pub file_config: FileConfig,
}

pub struct SearchConfig {
    pub query: String,
    pub preview: Option<usize>,
}

pub struct FileConfig {
    pub path: String,
    pub exclude_paths: Vec<String>,
    pub file_types: Option<Vec<String>>,
}

#[allow(dead_code)]
enum ArgumentTypes {
    ExcludePath,
    FileType,
    Preview,
}

enum Mode {
    Start,
    Argument(ArgumentTypes),
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut exclude_paths = Vec::new();
        let mut file_types = Vec::new();
        let mut mode = Mode::Start;
        let mut preview = None;
        args.next();

        if let Some(mut path) = args.next() {
            if let Some(query) = args.next() {
                if !path.ends_with('/') {
                    path.push('/');
                }

                for arg in args {
                    match mode {
                        Mode::Start => {
                            if arg == "-e" {
                                mode = Mode::Argument(ArgumentTypes::ExcludePath);
                            } else if arg == "-f" {
                                mode = Mode::Argument(ArgumentTypes::FileType);
                            } else if arg == "--preview" {
                                mode = Mode::Argument(ArgumentTypes::Preview);
                            }
                        }
                        Mode::Argument(t) => {
                            match t {
                                ArgumentTypes::ExcludePath => exclude_paths.push(arg),
                                ArgumentTypes::FileType => file_types.push(arg),
                                ArgumentTypes::Preview => preview = Some(arg.parse().unwrap_or(10)),
                            }
                            mode = Mode::Start;
                        }
                    }
                }

                let config = Config {
                    search_config: SearchConfig { query, preview },
                    file_config: FileConfig {
                        path,
                        exclude_paths,
                        file_types: if file_types.is_empty() {
                            None
                        } else {
                            Some(file_types)
                        },
                    },
                };

                return Ok(config);
            }
        }

        Err("Could not parse arguments.")
    }

    pub fn default() -> Config {
        let exclude_paths = [".git", "target", ".vscode"];
        Config {
            search_config: SearchConfig {
                query: "self".to_string(),
                preview: Some(10),
            },
            file_config: FileConfig {
                path: "./".to_string(),
                exclude_paths: exclude_paths.map(|x| x.to_string()).to_vec(),
                file_types: None,
            },
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Path: {}", self.file_config.path)?;
        writeln!(f, "Query: {}", self.search_config.query)?;
        writeln!(
            f,
            "exclude_paths: {}",
            self.file_config.exclude_paths.join(", ")
        )?;

        if let Some(file_types) = &self.file_config.file_types {
            writeln!(f, "fileTypes: {}", file_types.join(", "))?;
        }

        if let Some(preview) = self.search_config.preview {
            writeln!(f, "preview: {}", preview)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
