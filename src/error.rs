/// An error encountered during execution.
#[derive(Debug)]
pub enum Error {
    /// Failed to read the config file to a string.
    ReadConfig(std::io::Error),

    /// Failed to parse the config file.
    ParseConfig(toml::de::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReadConfig(e) => write!(f, "Failed to read config file: {e}"),
            Error::ParseConfig(e) => write!(f, "Failed to parse config file: {e}"),
        }
    }
}

impl std::error::Error for Error {}
