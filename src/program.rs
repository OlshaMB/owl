use crate::args::ProgramArguments;
use crate::lib::on_use::Use;
use colored::Colorize;
use permissions::is_readable;
use std::fs::read_to_string;
use std::io::stdin;
use std::process::exit;
impl Use for ProgramArguments {
    fn on_use(&self) {
        for file in &self.file {
            if file.exists() {
                if !is_readable(file).unwrap_or(false) {
                    println!(
                        "Cannot read file({}) no read permissions",
                        file.to_str().unwrap().bold()
                    );
                    exit(2);
                }
                print!("{}", read_to_string(file).unwrap());
            } else if file.to_str().unwrap() == "-" && cfg!(feature = "dash-in") {
                for line in stdin().lines() {
                    if line.is_err() {
                        continue;
                    }
                    println!("{}", line.unwrap());
                }
            } else {
                println!("File not found: {}", file.to_str().unwrap())
            }
        }
    }
}
