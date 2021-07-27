use crate::bytecode::chunk::OpCode;

use super::chunk::{Chunk, Value};

const STACK_MAX: usize = 256;

#[derive(Debug)]
struct VmStack<T> {
    stack: [T; STACK_MAX],
    stack_top: usize,
}

impl<T: Default + Copy> VmStack<T> {
    pub fn new() -> Self {
        Self {
            stack: [T::default(); STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), VmError> {
        if self.stack_top >= 256 {
            return Err(VmError::RuntimeError(RuntimeError::StackOverflow));
        }
        self.stack[self.stack_top] = val;
        self.stack_top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.stack_top <= 0 {
            return None;
        }

        self.stack_top -= 1;
        let val = self.stack[self.stack_top];
        Some(val)
    }
}

#[derive(Debug)]
struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: VmStack<Value>,
    debug_mode: bool,
}

#[derive(Debug)]
enum RuntimeError {
    StackOverflow,
}

#[derive(Debug)]
enum VmError {
    CompileError,
    RuntimeError(RuntimeError),
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 1,
            debug_mode: false,
            stack: VmStack::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), VmError> {
        loop {
            if let Some(instruction) = self.chunk.instruction_at(self.ip) {
                self.ip += 1;

                if self.debug_mode {
                    self.chunk.dissasemble_instruction(&instruction, 0, &mut 0);
                }

                let res = match instruction {
                    OpCode::Constant { constant_offset } => {
                        if let Some(the_constant) = self.chunk.read_constant(*constant_offset) {
                            println!("The constant is {:?}", the_constant);
                            Ok(())
                        } else {
                            Err(VmError::CompileError)
                        }
                    }
                    OpCode::Return => Ok(()),
                };

                return res;
            } else {
                return Err(VmError::CompileError);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_stack() {
        let mut stack = VmStack::<Value>::new();
        assert_eq!(stack.pop(), None);
        stack.push(63.2).unwrap();
        stack.push(6.2).unwrap();
        assert_eq!(stack.pop(), Some(6.2));
        assert_eq!(stack.pop(), Some(63.2));
        assert_eq!(stack.pop(), None);
    }
}
