use crate::vm::machine::{MAX_U15, Machine};
use std::env::args;
use std::fs::File;
use std::io::Read;

mod vm;

fn main() {
    let program_name = args()
        .skip(1)
        .next()
        .expect("Binary argument was not provided");
    let binary = std::fs::read(program_name).expect("Failed to read binary file");
    let mut machine = Machine::new(&binary);
    machine.run();
}
