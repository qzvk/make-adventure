#[derive(Debug)]
pub enum Error {
    /// Failed to lookup index of a page's link.
    BadIndex { page: String, link: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadIndex { page, link } => write!(
                f,
                "The page {page:?} links to {link:?}, which does not exist!"
            ),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn bad_index(page: impl Into<String>, link: impl Into<String>) -> Self {
        Error::BadIndex {
            page: page.into(),
            link: link.into(),
        }
    }
}
