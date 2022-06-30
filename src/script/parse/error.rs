use super::line::DirectiveKind;

#[derive(Debug)]
pub enum Error {
    InvalidIndentation { count: usize },
    UnexpectedIndenation { expected: usize, found: usize },
    UnexpectedArgument { block: DirectiveKind },
    ExcessiveChildCount { block: DirectiveKind },
    UnexpectedChildDirective { block: DirectiveKind },
    MissingArgument { block: DirectiveKind },
    MissingText { block: DirectiveKind },
    PageMissingTitle { page: String },
    ExcessivePageTitles { page: String },
    UnexpectedText,
    NestedPage { parent: String, child: String },
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
            Error::MissingText { block } => {
                write!(f, "A {block} directive has no text, but requires it.")
            }
            Error::UnexpectedArgument { block } => {
                write!(f, "A {block} directive cannot have an argument.")
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
            Error::MissingArgument { block } => {
                write!(f, "The {block} directive requires an argument.")
            }
            Error::PageMissingTitle { page } => {
                write!(f, "The page {page:?} was declared with no title.")
            }
            Error::ExcessivePageTitles { page } => {
                write!(f, "The page {page:?} has too many declared titles.")
            }
            Error::UnexpectedText => write!(f, "Unexpected text."),
            Error::NestedPage { parent, child } => {
                write!(f, "Page {child:?} is nested inside of {parent:?}.")
            }
        }
    }
}

impl std::error::Error for Error {}
