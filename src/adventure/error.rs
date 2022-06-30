#[derive(Debug)]
pub enum Error {
    BadReference { from: String, to: String },
}

impl Error {
    pub fn bad_reference(from: &str, to: &str) -> Self {
        Self::BadReference {
            from: from.to_owned(),
            to: to.to_owned(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadReference { from, to } => {
                write!(
                    f,
                    "The page {from:?} tries to link to page {to:?}, but it does not exist."
                )
            }
        }
    }
}

impl std::error::Error for Error {}
