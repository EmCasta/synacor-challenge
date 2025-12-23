use crate::vm::ram::Ram;

#[repr(u16)]
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Instruction {
    /// stop execution
    Halt = 0,
    /// set a b, set register a to the value of b
    Set(u16, u16) = 1,
    /// push <a> onto the stack
    Push(u16) = 2,
    /// remove the top element from the stack and write it into <a>; empty stack = error
    Pop(u16) = 3,
    /// set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    Eq(u16, u16, u16) = 4,
    /// set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    Gt(u16, u16, u16) = 5,
    /// jump to <a>
    Jmp(u16) = 6,
    /// if <a> is nonzero, jump to <b>
    Jnz(u16, u16) = 7,
    /// if <a> is zero, jump to <b>
    Jz(u16, u16) = 8,
    /// assign into <a> the sum of <b> and <c> (modulo 32768)
    Add(u16, u16, u16) = 9,
    /// store into <a> the product of <b> and <c> (modulo 32768)
    Mult(u16, u16, u16) = 10,
    /// store into <a> the remainder of <b> divided by <c>
    Mod(u16, u16, u16) = 11,
    /// stores into <a> the bitwise and of <b> and <c>
    And(u16, u16, u16) = 12,
    /// stores into <a> the bitwise or of <b> and <c>
    Or(u16, u16, u16) = 13,
    /// stores 15-bit bitwise inverse of <b> in <a>
    Not(u16, u16) = 14,
    /// read memory at address <b> and write it to <a>
    Rmem(u16, u16) = 15,
    /// write the value from <b> into memory at address <a>
    Wmem(u16, u16) = 16,
    /// write the address of the next instruction to the stack and jump to <a>
    Call(u16) = 17,
    /// remove the top element from the stack and jump to it; empty stack = halt
    Ret = 18,
    /// write the character represented by ascii code <a> to the terminal
    Out(u16) = 19,
    /// read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
    In(u16) = 20,
    /// no operation
    Noop = 21,
}

impl Instruction {
    /// Retorna la instruccion de la posición ptr de memoria, y incrementa el puntero segun la instruccion
    /// encontrada. Paniquea si la instrucción es inválida.
    pub fn parse_from_mem(memory: &mut Ram, ptr: &mut u16) -> Self {
        let instruction = memory.get_value(*ptr);
        match instruction {
            0 => {
                *ptr += 1;
                Self::Halt
            }
            1 => {
                // set <reg> b
                let reg = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                *ptr += 3;
                Self::Set(reg, b)
            }
            2 => {
                // push <a>
                let a = memory.get_value(*ptr + 1);
                *ptr += 2;
                Self::Push(a)
            }
            3 => {
                // pop a
                let a = memory.get_value(*ptr + 1);
                *ptr += 2;
                Self::Pop(a)
            }
            4 => {
                // eq a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::Eq(a, b, c)
            }
            5 => {
                // gt a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::Gt(a, b, c)
            }
            6 => {
                // jmp a
                let a = memory.get_value(*ptr + 1);
                *ptr += 2;
                Self::Jmp(a)
            }
            7 => {
                // jnz a b
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                *ptr += 3;
                Self::Jnz(a, b)
            }
            8 => {
                // jz a b
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                *ptr += 3;
                Self::Jz(a, b)
            }
            9 => {
                // add a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::Add(a, b, c)
            }
            10 => {
                // mult a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::Mult(a, b, c)
            }
            11 => {
                // mod a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::Mod(a, b, c)
            }
            12 => {
                // and a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::And(a, b, c)
            }
            13 => {
                // or a b c
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                let c = memory.get_value(*ptr + 3);
                *ptr += 4;
                Self::Or(a, b, c)
            }
            14 => {
                // not a b
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                *ptr += 3;
                Self::Not(a, b)
            }
            15 => {
                // rmem a b
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                *ptr += 3;
                Self::Rmem(a, b)
            }
            16 => {
                // wmem a b
                let a = memory.get_value(*ptr + 1);
                let b = memory.get_value(*ptr + 2);
                *ptr += 3;
                Self::Wmem(a, b)
            }
            17 => {
                // call a
                let a = memory.get_value(*ptr + 1);
                *ptr += 2;
                Self::Call(a)
            }
            18 => {
                // ret
                *ptr += 1;
                Self::Ret
            }
            19 => {
                // out a
                let a = memory.get_value(*ptr + 1);
                *ptr += 2;
                Self::Out(a)
            }
            20 => {
                // in a
                let a = memory.get_value(*ptr + 1);
                *ptr += 2;
                Self::In(a)
            }
            21 => {
                // noop
                *ptr += 1;
                Self::Noop
            }
            _ => panic!("Unknown instruction 0x{:x}, {}", instruction, instruction),
        }
    }
}
