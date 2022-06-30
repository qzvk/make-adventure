use super::line::DirectiveKind;

#[derive(Debug)]
pub enum Error {
    InvalidIndentation { count: usize },
    UnexpectedIndenation { expected: usize, found: usize },
    MissingTitleText,
    UnexpectedTitleArgument,
    ExcessiveChildCount { block: DirectiveKind },
    UnexpectedChildDirective { block: DirectiveKind },
    MissingLinkArgument,
    MissingLinkText,
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
                write!(f, "A title directive cannot have an argument.")
            }
            Error::ExcessiveChildCount { block } => {
                write!(f, "The {block} directive here expected a single child, but multiple were provided.")
            }
            Error::UnexpectedChildDirective { block } => {
                write!(
                    f,
                    "The {block} directive can only contain text, not other directives."
                )
            }
            Error::MissingLinkArgument => write!(f, "Link directives require an argument."),
            Error::MissingLinkText => write!(f, "Link directives require a line of text."),
        }
    }
}

impl std::error::Error for Error {}
