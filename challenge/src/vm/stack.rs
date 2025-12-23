/*
an unbounded stack which holds individual 16-bit values
*/

pub(crate) struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> u16 {
        let value = self.stack.pop();
        if let Some(value) = value {
            return value;
        }
        panic!("Empty stack");
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
