use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, process::ExitCode};

#[derive(Parser)]
struct Args {
    config: PathBuf,
    output: PathBuf,
}

/// A single page of the adventure.
#[derive(Debug, Deserialize)]
struct Page {
    /// The title of the page.
    pub title: String,

    /// The paragraphs of text within the page.
    pub paragraphs: Vec<String>,

    /// The links to other pages. Keys are user-presented strings. Values are page identifiers.
    pub links: HashMap<String, String>,
}

/// A configuration of an adventure.
#[derive(Debug, Deserialize)]
struct Config {
    /// The set of all pages of the adventure, keyed by a unique identifier.
    pages: HashMap<String, Page>,
}

/// An error encountered during execution.
#[derive(Debug)]
enum Error {
    /// Failed to read the config file to a string.
    ReadConfig(std::io::Error),

    /// Failed to parse the config file.
    ParseConfig(toml::de::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ReadConfig(e) => write!(f, "Failed to read config file: {e}"),
            Error::ParseConfig(e) => write!(f, "Failed to parse config file: {e}"),
        }
    }
}

impl std::error::Error for Error {}

fn run() -> Result<(), Error> {
    let args = Args::parse();
    let config_string = std::fs::read_to_string(args.config).map_err(Error::ReadConfig)?;
    let config: Config = toml::from_str(&config_string).map_err(Error::ParseConfig)?;

    println!("{config:?}");

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            drop(e);
            ExitCode::FAILURE
        }
    }
}
