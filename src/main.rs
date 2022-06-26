mod args;
mod config;
mod error;

use crate::{args::Args, config::Config, error::Error};
use handlebars::Handlebars;
use std::{path::PathBuf, process::ExitCode};

fn run() -> Result<(), Error> {
    use clap::Parser;

    let args = Args::parse();
    let config_string = std::fs::read_to_string(args.config).map_err(Error::ReadConfig)?;
    let config: Config = toml::from_str(&config_string).map_err(Error::ParseConfig)?;

    std::fs::create_dir_all(&args.output).map_err(Error::Directory)?;

    let template = std::fs::read_to_string(config.template).map_err(Error::ReadTemplate)?;

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("template", template)
        .map_err(Error::BadTemplate)?;

    for (identifier, page) in config.pages {
        let output = handlebars
            .render("template", &page)
            .map_err(Error::PageGeneration)?;

        let mut path = PathBuf::from(&args.output);
        path.push(format!("{identifier}.html"));

        std::fs::write(path, output).unwrap();
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
