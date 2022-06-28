use std::collections::HashMap;

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
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Script::new(value)
    }
}

impl Script {
    pub fn new(_string: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ParseError {}
