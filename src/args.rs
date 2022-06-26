use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the configuration file used to generate the adventure.
    pub config: PathBuf,

    /// The directory to write output files to.
    pub output: PathBuf,
}
