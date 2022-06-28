/// An error encountered during execution.
#[derive(Debug)]
pub enum Error {
    /// Failed to read the config file to a string.
    ReadConfig(std::io::Error),

    /// Failed to parse the config file.
    ParseConfig(toml::de::Error),

    /// Failed to read the script file to a string.
    ReadScript(std::io::Error),

    /// Failed to parse the script.
    ParseScript(toml::de::Error),

    /// Failed to create output directory.
    Directory(std::io::Error),

    /// Failed to read the template file to a string.
    ReadTemplate(std::io::Error),

    /// A template was not valid.
    BadTemplate(handlebars::TemplateError),

    /// Failed to generate an adventure with the given config.
    Adventure(crate::adventure::error::Error),

    /// Page generation failed.
    PageGeneration(handlebars::RenderError),

    /// Failed to write an output file.
    WriteOutput(std::path::PathBuf, std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReadConfig(e) => write!(f, "Failed to read config file: {e}"),
            Error::ParseConfig(e) => write!(f, "Failed to parse config file: {e}"),
            Error::ReadScript(e) => write!(f, "Failed to read script file: {e}"),
            Error::ParseScript(e) => write!(f, "Failed to parse script file: {e}"),
            Error::Directory(e) => write!(f, "Failed to create output directory: {e}"),
            Error::ReadTemplate(e) => write!(f, "Failed to read template file: {e}"),
            Error::BadTemplate(e) => write!(f, "Failed parse template file: {e}"),
            Error::Adventure(e) => write!(f, "Failed to generate adventure from config: {e}"),
            Error::PageGeneration(e) => write!(f, "Failed generate a page: {e}"),
            Error::WriteOutput(name, e) => write!(
                f,
                "Failed to write file {name:?} to the output directory: {e}"
            ),
        }
    }
}

impl std::error::Error for Error {}
