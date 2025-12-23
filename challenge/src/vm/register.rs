use std::fmt::{Display, Formatter};
use crate::vm::machine::{CANT_REGS, MAX_U15};

/*
- Encountering a register as an operation argument should be taken as reading from the register or
  setting into the register as appropriate.
*/

pub(crate) struct Register {
    data: u16,
    number: u8,
}

impl Register {
    pub fn new(number: u8) -> Self {
        Self { data: 0, number }
    }

    pub fn store_data(&mut self, data: u16) {
        self.data = data;
    }

    pub fn get_data(&self) -> u16 {
        self.data
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register {}, data: {}", self.number, self.data)
    }
}