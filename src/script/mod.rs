pub mod parse;

/// A single page of the adventure.
#[derive(Debug)]
pub struct Page<'a> {
    /// String used to identify page.
    pub identifier: &'a str,

    /// The title of the page.
    pub title: &'a str,

    /// The paragraphs of text within the page.
    pub paragraphs: Vec<&'a str>,

    /// The links to other pages, the target page identifier and user-facing text.
    pub links: Vec<(&'a str, &'a str)>,
}

/// A configuration of an adventure.
#[derive(Debug)]
pub struct Script<'a> {
    /// The list of all pages of the adventure.
    pub pages: Vec<Page<'a>>,
}

impl<'a> Script<'a> {
    pub fn new(string: &'a str) -> Result<Self, Vec<(usize, parse::Error)>> {
        parse::parse(string)
    }
}
