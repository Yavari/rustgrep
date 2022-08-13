use std::fmt;

pub struct Config {
    pub query: String,
    pub path: String,
    pub exclude_paths: Vec<String>,
}

#[allow(dead_code)]
enum ArgumentTypes {
    ExcludePath,
    SomeOtherCommand,
}

enum Mode {
    Start,
    Argument(ArgumentTypes),
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut exclude_paths = Vec::new();
        let mut mode = Mode::Start;
        args.next();

        if let Some(mut path) = args.next() {
            if let Some(query) = args.next() {
                if !path.ends_with('/') {
                    path.push('/');
                }

                for arg in args {
                    match mode {
                        Mode::Start => {
                            let mut chars = arg.chars();
                            if let Some(a) = chars.next() {
                                if let Some(b) = chars.next() {
                                    if a == '-' && b == 'e' {
                                        mode = Mode::Argument(ArgumentTypes::ExcludePath);
                                    }
                                }
                            }
                        }
                        Mode::Argument(t) => match t {
                            ArgumentTypes::ExcludePath => {
                                exclude_paths.push(arg);
                                mode = Mode::Start;
                            }
                            ArgumentTypes::SomeOtherCommand => todo!(),
                        },
                    }
                }

                let c = Config {
                    path,
                    query,
                    exclude_paths,
                };

                return Ok(c);
            }
        }

        Err("Could not parse arguments.")
    }

    pub fn default() -> Config {
        let exclude_paths = [".git", "target", ".vscode"];
        Config {
            path: "./".to_string(),
            query: "self".to_string(),
            exclude_paths: exclude_paths.map(|x| x.to_string()).to_vec(),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Path: {}", self.path)?;
        writeln!(f, "Query: {}", self.query)?;
        writeln!(f, "exclude_paths: {}", self.exclude_paths.join(", "))?;
        writeln!(f, "")?;
        Ok(())
    }
}
