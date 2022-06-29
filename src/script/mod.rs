use std::collections::HashMap;

pub mod parse;

/// A single page of the adventure.
#[derive(Debug)]
pub struct Page {
    /// The title of the page.
    pub title: String,

    /// The paragraphs of text within the page.
    pub paragraphs: Vec<String>,

    /// The links to other pages. Keys are user-presented strings. Values are page indicies.
    pub links: HashMap<String, usize>,
}

/// A configuration of an adventure.
#[derive(Debug)]
pub struct Script {
    /// The list of all pages of the adventure.
    pub pages: Vec<Page>,
}

impl Script {
    #[inline]
    pub fn new(string: &str) -> Result<Self, Vec<(usize, parse::Error)>> {
        parse::parse(string)
    }
}
