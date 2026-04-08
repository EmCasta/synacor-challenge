use crate::vm::machine::Machine;
use std::env::args;

mod vm;

fn main() {
    let program_name = args().nth(1).expect("Binary argument was not provided");
    let binary = std::fs::read(program_name).expect("Failed to read binary file");
    let mut machine = Machine::new(&binary);
    machine.run();
}
