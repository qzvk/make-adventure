use std::collections::HashMap;

pub mod parse;

/// A single page of the adventure.
#[derive(Debug)]
pub struct Page {
    /// The title of the page.
    pub title: String,

    /// The paragraphs of text within the page.
    pub paragraphs: Vec<String>,

    /// The links to other pages. Keys are user-presented strings. Values are page identifiers.
    pub links: HashMap<String, String>,
}

/// A configuration of an adventure.
#[derive(Debug)]
pub struct Script {
    /// The set of all pages of the adventure, keyed by a unique identifier.
    pub pages: HashMap<String, Page>,
}

impl TryFrom<&str> for Script {
    type Error = parse::Error;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Script::new(value)
    }
}

impl Script {
    #[inline]
    pub fn new(string: &str) -> Result<Self, parse::Error> {
        parse::parse(string)
    }
}
