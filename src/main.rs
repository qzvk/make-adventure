mod adventure;
mod args;
mod config;
mod error;

use crate::{adventure::Adventure, args::Args, config::Config, error::Error};
use clap::Parser;
use handlebars::Handlebars;
use std::{path::PathBuf, process::ExitCode};

/// Read and parse the config file named by the arguments.
fn get_config(args: &Args) -> Result<Config, Error> {
    let string = std::fs::read_to_string(&args.config).map_err(Error::ReadConfig)?;
    let config = toml::from_str(&string).map_err(Error::ParseConfig)?;
    Ok(config)
}

/// Create the output directory (if it does not already exist).
fn create_output_dir(args: &Args) -> Result<(), Error> {
    std::fs::create_dir_all(&args.output).map_err(Error::Directory)
}

/// Create and configure a handlebars instance from the given config.
fn create_handlebars(config: &Config) -> Result<Handlebars, Error> {
    let template = std::fs::read_to_string(&config.template).map_err(Error::ReadTemplate)?;

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("template", template)
        .map_err(Error::BadTemplate)?;

    Ok(handlebars)
}

/// Generate and write a page to the output directory.
fn generate_page(
    args: &Args,
    handlebars: &Handlebars,
    index: usize,
    page: &adventure::Page,
) -> Result<(), Error> {
    let output = handlebars
        .render("template", &page)
        .map_err(Error::PageGeneration)?;

    let mut path = PathBuf::from(&args.output);
    // Indicies offset by 1, since they're read by humans.
    path.push(format!("{}.html", index + 1));

    match std::fs::write(&path, output) {
        Ok(()) => {
            println!("Wrote {path:?}");
            Ok(())
        }
        Err(e) => Err(Error::WriteOutput(path, e)),
    }
}

/// Copy additional files mentioned by the config to the output directory.
fn copy_additional_files(args: &Args, config: &Config) -> Result<(), Error> {
    if let Some(additional_files) = &config.additional_files {
        for file in additional_files {
            let mut destination = PathBuf::from(&args.output);
            destination.push(&file);

            std::fs::copy(&file, &destination).map_err(|e| Error::WriteOutput(destination, e))?;
            println!("Copied {file:?}");
        }
    }

    Ok(())
}

fn run() -> Result<(), Error> {
    let args = Args::parse();
    let config = get_config(&args)?;

    create_output_dir(&args)?;

    let handlebars = create_handlebars(&config)?;

    let adventure = Adventure::new(&config).map_err(Error::Adventure)?;

    for (index, page) in adventure.pages.iter().enumerate() {
        generate_page(&args, &handlebars, index, page)?;
    }

    copy_additional_files(&args, &config)
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
