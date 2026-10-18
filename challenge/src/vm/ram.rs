use crate::vm::machine::MAX_U15;

/// Memory in the VM: it has a 15-bit address space, storing 16-bit values. The values are stored as a 16-bit
/// little-endian pair (low byte, high byte).
pub(crate) struct Ram {
    memory: [u8; (MAX_U15 + 1) * 2],
}

impl Ram {
    /// Creates a new Ram, initializing its values to 0
    pub fn new() -> Self {
        Self {
            memory: [0_u8; (MAX_U15 + 1) * 2],
        }
    }

    /// Loads a program into memory, copying it byte by byte from address 0 to len(program).
    ///
    /// ## Panics
    /// If the program does not fit into memory, it panics
    pub fn load_program(&mut self, program: &[u8]) {
        if program.len() > self.memory.len() {
            panic!("Program does not fit into memory!");
        }
        self.memory[..program.len()].copy_from_slice(program);
    }

    /// Returns the 16-bit value stored at a given address
    pub fn get_value(&self, address: u16) -> u16 {
        let addr = address as u32 * 2;
        let low = self.memory[addr as usize] as u16;
        let high = self.memory[addr as usize + 1] as u16;
        (high << 8) + low
    }

    /// Stores a value at a given address in memory
    pub fn store_value(&mut self, address: u16, value: u16) {
        let addr = address as u32 * 2;
        let low = (value % 256) as u8;
        let high = (value >> 8) as u8;
        self.memory[addr as usize] = low;
        self.memory[addr as usize + 1] = high;
    }
}
