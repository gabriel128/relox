use crate::bytecode::chunk::OpCode;

use super::chunk::{Chunk, Value};

const STACK_MAX: usize = 256;

#[derive(Debug)]
enum RuntimeError {
    StackOverflow,
    GenericError,
}

#[derive(Debug)]
enum VmError {
    CompileError,
    RuntimeError(RuntimeError),
}

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

    pub fn pop(&mut self) -> Result<T, VmError> {
        if self.stack_top <= 0 {
            return Err(VmError::CompileError);
        }

        self.stack_top -= 1;
        let val = self.stack[self.stack_top];
        Ok(val)
    }
}

#[derive(Debug)]
struct Vm {
    chunk: Chunk,
    ip: usize,
    instr_stack: VmStack<Value>,
    debug_mode: bool,
}

impl Vm {
    pub fn new(chunk: Chunk, debug_mode: bool) -> Self {
        Self {
            chunk,
            debug_mode,
            ip: 0,
            instr_stack: VmStack::new(),
        }
    }

    pub fn run(&mut self) -> Result<Value, VmError> {
        loop {
            if let Some(instruction) = self.chunk.instruction_at(self.ip) {
                self.ip += 1;

                if self.debug_mode {
                    println!("== Current stack ==");
                    dbg!(&self.instr_stack);
                    self.chunk.dissasemble_instruction(&instruction, 0, &mut 0);
                }

                match instruction {
                    OpCode::Constant { constant_offset } => {
                        let the_constant = self
                            .chunk
                            .read_constant(*constant_offset)
                            .ok_or_else(|| VmError::CompileError)?;
                        self.instr_stack.push(*the_constant)?;
                    }
                    OpCode::Negate => {
                        let value = self.instr_stack.pop()?;
                        self.instr_stack.push(-value)?;
                    }
                    OpCode::Add => self.binary_op(std::ops::Add::add)?,
                    OpCode::Substract => self.binary_op(std::ops::Sub::sub)?,
                    OpCode::Divide => self.binary_op(std::ops::Div::div)?,
                    OpCode::Multiply => self.binary_op(std::ops::Mul::mul)?,
                    OpCode::Return => {
                        let value = self.instr_stack.pop()?;
                        println!("Return Value is: {:?}", value);
                        return Ok(value);
                    }
                };
            } else {
                return Err(VmError::CompileError);
            }
        }
    }

    fn binary_op<F>(&mut self, mut op: F) -> Result<(), VmError>
    where
        F: FnMut(Value, Value) -> Value,
    {
        let x = self.instr_stack.pop()?;
        let y = self.instr_stack.pop()?;
        self.instr_stack.push(op(x, y))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_stack() {
        let mut stack = VmStack::<Value>::new();
        assert!(stack.pop().is_err());
        stack.push(63.2).unwrap();
        stack.push(6.2).unwrap();
        assert_eq!(stack.pop().unwrap(), 6.2);
        assert_eq!(stack.pop().unwrap(), 63.2);
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_negation() {
        let mut chunk = Chunk::new();
        chunk.add_constant(3.0, 0).unwrap();
        chunk.write_bytecode(OpCode::Negate, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), -3.0);
    }

    #[test]
    fn test_addition() {
        let mut chunk = Chunk::new();
        chunk.add_constant(3.0, 0).unwrap();
        chunk.add_constant(2.0, 0).unwrap();
        chunk.write_bytecode(OpCode::Add, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), 5.0);
    }

    #[test]
    fn test_mult() {
        let mut chunk = Chunk::new();
        chunk.add_constant(3.0, 0).unwrap();
        chunk.add_constant(2.0, 0).unwrap();
        chunk.write_bytecode(OpCode::Multiply, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), 6.0);
    }
}