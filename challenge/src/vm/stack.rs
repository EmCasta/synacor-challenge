/// An unbounded stack which holds individual 16-bit values
pub(crate) struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    /// Creates a new empty stack
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes a value on the top of the stack.
    pub fn push(&mut self, value: u16) {
        self.stack.push(value);
    }

    /// Removes the value from the top of the stack. If the stack was empty, it panics.
    pub fn pop(&mut self) -> u16 {
        let value = self.stack.pop();
        if let Some(value) = value {
            return value;
        }
        panic!("Empty stack");
    }

    /// Returns true if the stack is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

mod tests {


    #[test]
    fn can_create() {
        let stack = super::Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn can_push_and_pop() {
        let mut stack = super::Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    fn can_test_if_stack_is_empty() {
        let mut stack = super::Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    #[should_panic]
    fn panics_if_popped_when_empty() {
        let mut stack = super::Stack::new();
        stack.pop();
    }
}
