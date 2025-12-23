use std::io::Read;
use crate::vm::instruction::Instruction;
use crate::vm::ram::Ram;
use crate::vm::register::Register;
use crate::vm::stack::Stack;

/*
- three storage regions
  - memory with 15-bit address space storing 16-bit values
  - eight registers
  - an unbounded stack which holds individual 16-bit values
- all numbers are unsigned integers 0..32767 (15-bit)
- all math is modulo 32768; 32758 + 15 => 5

- numbers 0..32767 mean a literal value
- numbers 32768..32775 instead mean registers 0..7
- numbers 32776..65535 are invalid
*/

pub const CANT_REGS: usize = 8;
pub const MAX_U15: usize = 32767;

pub struct Machine {
    instruction_ptr: u16,
    ram: Ram,
    stack: Stack,
    registers: [Register; CANT_REGS],
    is_running: bool,
}

impl Machine {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = Ram::new();
        memory.load_program(program);
        let registers = [
            Register::new(0),
            Register::new(1),
            Register::new(2),
            Register::new(3),
            Register::new(4),
            Register::new(5),
            Register::new(6),
            Register::new(7),
        ];
        Self {
            instruction_ptr: 0,
            ram: memory,
            stack: Stack::new(),
            registers,
            is_running: true,
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
            let instruction = Instruction::parse_from_mem(&mut self.ram, &mut self.instruction_ptr);
            //println!("Instruccion: {:?}", instruction);
            //println!("Initial dump");
            //self.dump_state();
            match instruction {
                Instruction::Halt => {
                    self.is_running = false;
                }
                Instruction::Noop => {
                    // no hacer nada :)
                }
                Instruction::Out(c) => {
                    let character = self.get_value(c) as u8 as char;
                    print!("{}", character);
                },
                Instruction::Set(reg, b) => {
                    if !Self::is_valid_register(reg) {
                        panic!("Invalid register: {}", reg);
                    }
                    let value = self.get_value(b);
                    self.registers[reg as usize - MAX_U15 - 1].store_data(value);
                },
                Instruction::Push(a) => {
                    let value = self.get_value(a);
                    self.stack.push(value);
                },
                Instruction::Pop(a) => {
                    let value = self.stack.pop();
                    self.set_value(a, value);
                },
                Instruction::Eq(a, b, c) => {
                    let b = self.get_value(b);
                    let c = self.get_value(c);
                    if b == c {
                        self.set_value(a, 1);
                    } else {
                        self.set_value(a, 0);
                    }
                },
                Instruction::Gt(a, b, c) => {
                    let b = self.get_value(b);
                    let c = self.get_value(c);
                    if b > c {
                        self.set_value(a, 1);
                    } else {
                        self.set_value(a, 0);
                    }
                },
                Instruction::Jmp(a) => {
                    let a = self.get_value(a);
                    self.instruction_ptr = a % (MAX_U15 + 1) as u16;
                },
                Instruction::Jnz(a, b) => {
                    let a = self.get_value(a);
                    let b = self.get_value(b);
                    if a != 0 {
                        self.instruction_ptr = b % (MAX_U15 + 1) as u16;
                    }
                },
                Instruction::Jz(a, b) => {
                    let a = self.get_value(a);
                    let b = self.get_value(b);
                    if a == 0 {
                        self.instruction_ptr = b % (MAX_U15 + 1) as u16;
                    }
                },
                Instruction::Add(a, b, c) => {
                    let b = self.get_value(b);
                    let c = self.get_value(c);
                    let sum = (b + c) % (MAX_U15 + 1) as u16;
                    self.set_value(a, sum);
                },
                Instruction::Mult(a, b, c) => {
                    let b = self.get_value(b) as u32;
                    let c = self.get_value(c) as u32;
                    let mult = (b * c) % (MAX_U15 + 1) as u32;
                    self.set_value(a, mult as u16);
                },
                Instruction::Mod(a, b, c) => {
                    let b = self.get_value(b);
                    let c = self.get_value(c);
                    let modulo = (b % c) % (MAX_U15 + 1) as u16;
                    self.set_value(a, modulo);
                },
                Instruction::And(a, b, c) => {
                    let b = self.get_value(b);
                    let c = self.get_value(c);
                    let and = (b & c);
                    self.set_value(a, and);
                },
                Instruction::Or(a, b, c) => {
                    let b = self.get_value(b);
                    let c = self.get_value(c);
                    let or = (b | c);
                    self.set_value(a, or);
                },
                Instruction::Not(a, b) => {
                    let b = self.get_value(b);
                    let not = !b % (MAX_U15 + 1) as u16;
                    self.set_value(a, not);
                },
                Instruction::Rmem(a, b) => {
                    let addr = self.get_value(b);
                    let value = self.ram.get_value(addr);
                    self.set_value(a, value);
                },
                Instruction::Wmem(a, b) => {
                    let b = self.get_value(b);
                    let value = self.get_value(a);
                    self.set_value(value, b);
                },
                Instruction::Call(a) => {
                    let next = self.instruction_ptr;    // ptr ya se incremento en parse_from_mem
                    let a = self.get_value(a);
                    self.stack.push(next);
                    self.instruction_ptr = a % (MAX_U15 + 1) as u16;
                },
                Instruction::Ret => {
                    if self.stack.is_empty() {
                        self.is_running = false;
                    } else {
                        let next = self.stack.pop();
                        self.instruction_ptr = next % (MAX_U15 + 1) as u16;
                    }
                },
                Instruction::In(a) => {
                    let mut buf = [0_u8; 1];
                    let _ = std::io::stdin().read_exact(&mut buf).expect("Failed to read character from stdin");
                    self.set_value(a, buf[0] as u16);
                }
            }
            //println!("After dump");
            //self.dump_state();
            //println!();
        }
    }

    pub fn dump_state(&self) {
        println!("Instruction ptr: {}", self.instruction_ptr);
        println!("Running: {}", self.is_running);
        for r in &self.registers {
            println!("{}", r);
        }
    }

    pub fn is_literal(address: u16) -> bool {
        (0..MAX_U15 + 1).contains(&(address as usize))
    }

    pub fn is_valid_register(number: u16) -> bool {
        (MAX_U15 + 1..MAX_U15 + CANT_REGS + 1).contains(&(number as usize))
    }

    pub fn get_value(&self, number: u16) -> u16 {
        if Self::is_literal(number) {
            number
        } else if Self::is_valid_register(number) {
            self.registers[number as usize - MAX_U15 - 1].get_data()
        } else {
            panic!("Invalid number: {}", number);
        }
    }

    pub fn set_value(&mut self, address: u16, value: u16) {
        if Self::is_literal(address) {    // se considera que es memoria (15 bits)
            self.ram.store_value(address, value);
        } else if Self::is_valid_register(address) {
            self.registers[address as usize - MAX_U15 - 1].store_data(value);
        } else {
            panic!("Invalid number: {}", address);
        }
    }
}
