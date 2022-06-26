use serde::Deserialize;
use std::collections::HashMap;

/// A single page of the adventure.
#[derive(Debug, Deserialize)]
pub struct Page {
    /// The title of the page.
    pub title: String,

    /// The paragraphs of text within the page.
    pub paragraphs: Vec<String>,

    /// The links to other pages. Keys are user-presented strings. Values are page identifiers.
    pub links: HashMap<String, String>,
}

/// A configuration of an adventure.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The set of all pages of the adventure, keyed by a unique identifier.
    pages: HashMap<String, Page>,
}
