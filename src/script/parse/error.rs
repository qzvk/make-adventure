use super::line::DirectiveKind;

#[derive(Debug)]
pub enum Error {
    InvalidIndentation { count: usize },
    UnexpectedIndenation { expected: usize, found: usize },
    TextAtTopLevel,
    UnexpectedTopLevelDirective { found: DirectiveKind },
    MissingTitleDirective { page: String },
    MissingPageName,
    MissingTitleText { page: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidIndentation { count } => write!(
                f,
                "Invalid indentation, expected a multiple of four spaces, but saw {count}."
            ),
            Error::UnexpectedIndenation { expected, found } => write!(
                f,
                "Unexpected indentation, expected {expected}, but saw {found}."
            ),
            Error::TextAtTopLevel => write!(
                f,
                "Unexpected text. Only page declarations are allowed at the top level."
            ),
            Error::UnexpectedTopLevelDirective { found } => write!(
                f,
                "Unexpected {found} directive. Only page declarations are allowed at the top level."
            ),
            Error::MissingTitleDirective { page } => {
                write!(f, "The page {page:?} doesn't have a title.")
            }
            Error::MissingPageName => write!(f, "The page was declared with no name."),
            Error::MissingTitleText { page } => {
                write!(
                    f,
                    "The page {page:?} has a title directive, but no text for it."
                )
            }
        }
    }
}

impl std::error::Error for Error {}
