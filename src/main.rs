use clap::Parser;

mod args;
mod lib;
mod program;

use crate::args::ProgramArguments;
use crate::lib::on_use::Use;

fn main() {
    let program_arguments = ProgramArguments::parse();
    program_arguments.on_use();
}
