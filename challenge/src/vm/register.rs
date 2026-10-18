use std::fmt::{Display, Formatter};

/// Register on the VM: it has a number and holds individual 16-bit values
pub(crate) struct Register {
    data: u16,
    number: u8,
}

impl Register {
    /// Creates a new register, setting a number as the id of the register, and initializing its
    /// value to 0
    pub fn new(number: u8) -> Self {
        Self { data: 0, number }
    }

    /// Changes the contents of the register to the new data
    pub fn store_data(&mut self, data: u16) {
        self.data = data;
    }

    /// Returns tha value stored in the register
    pub fn get_data(&self) -> u16 {
        self.data
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register {}, data: {}", self.number, self.data)
    }
}
