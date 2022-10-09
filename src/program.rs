use crate::args::ProgramArguments;
use crate::lib::on_use::Use;
use std::fs::read_to_string;
impl Use for ProgramArguments {
    fn on_use(&self) -> u8 {
        for file in &self.file {
            if file.exists() {
                print!("{}", read_to_string(file).unwrap());
            }
        }
        return 0;
    }
}
