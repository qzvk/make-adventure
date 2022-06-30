use super::line::DirectiveKind;

#[derive(Debug)]
pub enum Error {
    InvalidIndentation { count: usize },
    UnexpectedIndenation { expected: usize, found: usize },
    MissingTitleText,
    UnexpectedTitleArgument,
    ExcessiveTitleText,
    UnexpectedChildDirectiveOfTitle,
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
            Error::MissingTitleText => write!(f, "A title directive has no text for it."),
            Error::UnexpectedTitleArgument => {
                write!(f, "A title declaration cannot have an argument.")
            }
            Error::ExcessiveTitleText => {
                write!(f, "Title declarations can only contain one line of text.")
            }
            Error::UnexpectedChildDirectiveOfTitle => {
                write!(
                    f,
                    "Title declaraions can only contain text, not other directives."
                )
            }
        }
    }
}

impl std::error::Error for Error {}
