mod args;
mod config;
mod error;

use crate::{args::Args, config::Config, error::Error};
use std::process::ExitCode;

fn run() -> Result<(), Error> {
    use clap::Parser;

    let args = Args::parse();
    let config_string = std::fs::read_to_string(args.config).map_err(Error::ReadConfig)?;
    let config: Config = toml::from_str(&config_string).map_err(Error::ParseConfig)?;

    for (identifier, page) in config.pages {
        let mut string = String::new();
        string += &format!("ID: {:?}\n", identifier);
        string += &format!("Title: {:?}\n", page.title);
        string += "Paragraphs:\n";

        for (i, paragraph) in page.paragraphs.iter().enumerate() {
            string += &format!("    [{}] {:?}\n", i, paragraph);
        }

        string += "Links:\n";

        for (i, (text, link)) in page.links.iter().enumerate() {
            string += &format!("    [{}] {:?} to {:?}\n", i, text, link);
        }

        println!("{}", string);
    }

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
