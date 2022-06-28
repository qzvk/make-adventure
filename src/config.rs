use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A configuration of an adventure.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "kebab-case")]
pub struct Config {
    /// The path of the template file to use.
    pub template: PathBuf,

    /// Additional files to copy to output directory.
    pub additional_files: Option<Vec<PathBuf>>,

    /// The path of the script file to use.
    pub script: PathBuf,
}
