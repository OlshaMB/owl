use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProgramArguments {
    pub file: Vec<PathBuf>,
}
