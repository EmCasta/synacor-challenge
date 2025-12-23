use crate::vm::machine::MAX_U15;

/*
memory with 15-bit address space storing 16-bit values

each number is stored as a 16-bit little-endian pair (low byte, high byte)
*/

pub(crate) struct Ram {
    memory: [u8; (MAX_U15 + 1) * 2],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            memory: [0_u8; (MAX_U15 + 1) * 2],
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        if program.len() > self.memory.len() {
            panic!("Program does not fit into memory!");
        }
        for idx in 0..program.len() {
            self.memory[idx] = program[idx];
        }
    }

    pub fn get_value(&self, address: u16) -> u16 {
        let addr = address as u32 * 2;
        let low = self.memory[addr as usize] as u16;
        let high = self.memory[addr as usize + 1] as u16;
        (high << 8) + low
    }

    pub fn store_value(&mut self, address: u16, value: u16) {
        let addr = address as u32 * 2;
        let low = (value % 256) as u8;
        let high = (value >> 8) as u8;
        self.memory[addr as usize] = low;
        self.memory[addr as usize + 1] = high;
    }
}
