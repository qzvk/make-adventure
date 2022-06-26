/// An error encountered during execution.
#[derive(Debug)]
pub enum Error {
    /// Failed to read the config file to a string.
    ReadConfig(std::io::Error),

    /// Failed to parse the config file.
    ParseConfig(toml::de::Error),

    /// Failed to create output directory.
    Directory(std::io::Error),

    /// Failed to read the template file to a string.
    ReadTemplate(std::io::Error),

    /// A template was not valid.
    BadTemplate(handlebars::TemplateError),

    /// Failed to generate an adventure with the given config.
    Adventure(crate::adventure::Error),

    /// Page generation failed.
    PageGeneration(handlebars::RenderError),

    /// Failed to copy an additional file.
    CopyAdditionalFile(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReadConfig(e) => write!(f, "Failed to read config file: {e}"),
            Error::ParseConfig(e) => write!(f, "Failed to parse config file: {e}"),
            Error::Directory(e) => write!(f, "Failed to create output directory: {e}"),
            Error::ReadTemplate(e) => write!(f, "Failed to read template file: {e}"),
            Error::BadTemplate(e) => write!(f, "Failed parse template file: {e}"),
            Error::Adventure(e) => write!(f, "Failed to generate adventure from config: {e}"),
            Error::PageGeneration(e) => write!(f, "Failed generate a page: {e}"),
            Error::CopyAdditionalFile(e) => write!(
                f,
                "Failed to copy an additional file to the output directory: {e}"
            ),
        }
    }
}

impl std::error::Error for Error {}
