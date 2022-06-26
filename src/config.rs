use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// A single page of the adventure.
#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    /// The title of the page.
    pub title: String,

    /// The paragraphs of text within the page.
    pub paragraphs: Vec<String>,

    /// The links to other pages. Keys are user-presented strings. Values are page identifiers.
    pub links: HashMap<String, String>,
}

/// A configuration of an adventure.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "kebab-case")]
pub struct Config {
    /// The path of the template file to use.
    pub template: PathBuf,

    /// Additional files to copy to output directory.
    pub additional_files: Option<Vec<PathBuf>>,

    /// The set of all pages of the adventure, keyed by a unique identifier.
    pub pages: HashMap<String, Page>,
}
