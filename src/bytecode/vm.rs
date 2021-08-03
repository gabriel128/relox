use crate::{
    bytecode::chunk::OpCode,
    errors::{
        ErrorKind::{RuntimeError, StackOverFlow},
        ReloxError,
    },
};

use super::{chunk::Chunk, value::Value};
use crate::Result;

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

    pub fn push(&mut self, val: T) -> Result<()> {
        if self.stack_top >= 256 {
            return Err(ReloxError::new_runtime_error(
                0,
                "StackOverflow bro".to_string(),
                StackOverFlow,
            ));
        }
        self.stack[self.stack_top] = val;
        self.stack_top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T> {
        if self.stack_top <= 0 {
            return Err(ReloxError::new_fatal_error(
                "Tried to pop invalid index from instruction stack".to_string(),
            ));
        }

        self.stack_top -= 1;
        let val = self.stack[self.stack_top];
        Ok(val)
    }

    pub fn stack_slice(&self, from: usize, to: usize) -> &[T] {
        &self.stack[from..to]
    }
}

#[derive(Debug)]
pub struct Vm {
    chunk: Chunk,
    ip: usize,
    instr_stack: VmStack<Value>,
    debug_mode: bool,
}

impl Vm {
    pub fn run_with(chunk: Chunk, debug_mode: bool) -> Result<Value> {
        Self::new(chunk, debug_mode).run()
    }

    pub fn new(chunk: Chunk, debug_mode: bool) -> Self {
        Self {
            chunk,
            debug_mode,
            ip: 0,
            instr_stack: VmStack::new(),
        }
    }

    pub fn run(&mut self) -> Result<Value> {
        loop {
            if let Some(instruction) = self.chunk.instruction_at(self.ip) {
                self.ip += 1;

                if self.debug_mode {
                    println!("== Current stack ==");
                    println!("{:?}", &self.instr_stack.stack_slice(0, self.ip + 1));
                    self.chunk.dissasemble_instruction(&instruction, 0, &mut 0);
                }

                match instruction {
                    OpCode::Constant { constant_offset } => {
                        let the_constant =
                            self.chunk.read_constant(*constant_offset).ok_or_else(|| {
                                ReloxError::new_fatal_error("Constant not set".to_string())
                            })?;
                        self.instr_stack.push(*the_constant)?;
                    }
                    OpCode::Negate => {
                        let value = self.instr_stack.pop()?;
                        match -value {
                            Ok(neg_value) => self.instr_stack.push(neg_value)?,
                            Err(error_msg) => {
                                let line_num = self.chunk.line_at(self.ip - 1);
                                return Err(ReloxError::new_runtime_error(
                                    line_num as usize,
                                    error_msg.to_string(),
                                    RuntimeError,
                                ));
                            }
                        };
                    }
                    OpCode::Add => self.binary_op(std::ops::Add::add)?,
                    OpCode::Substract => self.binary_op(std::ops::Sub::sub)?,
                    OpCode::Divide => self.binary_op(std::ops::Div::div)?,
                    OpCode::Multiply => self.binary_op(std::ops::Mul::mul)?,
                    OpCode::Return => {
                        let value = self.instr_stack.pop()?;
                        return Ok(value);
                    }
                };
            } else {
                return Err(ReloxError::new_fatal_error(format!(
                    "Read wrong instruction, stacktrace: {:?}",
                    &self.instr_stack.stack_slice(0, self.ip + 1)
                )));
            }
        }
    }

    fn binary_op<F>(&mut self, mut op: F) -> Result<()>
    where
        F: FnMut(Value, Value) -> Result<Value>,
    {
        let x = self.instr_stack.pop()?;
        let y = self.instr_stack.pop()?;
        match op(y, x) {
            Ok(value) => self.instr_stack.push(value),
            Err(error_msg) => {
                let line_num = self.chunk.line_at(self.ip - 1);
                Err(ReloxError::new_runtime_error(
                    line_num as usize,
                    error_msg.to_string(),
                    RuntimeError,
                ))
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
        assert!(stack.pop().is_err());
        stack.push(Value::Number(63.2)).unwrap();
        stack.push(Value::Number(6.2)).unwrap();
        assert_eq!(stack.pop().unwrap(), Value::Number(6.2));
        assert_eq!(stack.pop().unwrap(), Value::Number(63.2));
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_negation() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(3.0), 0).unwrap();
        chunk.write_bytecode(OpCode::Negate, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), Value::Number(-3.0));
    }

    #[test]
    fn test_addition() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(3.0), 0).unwrap();
        chunk.add_constant(Value::Number(2.0), 0).unwrap();
        chunk.write_bytecode(OpCode::Add, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), Value::Number(5.0));
    }

    #[test]
    fn test_subsraction() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(3.0), 0).unwrap();
        chunk.add_constant(Value::Number(2.0), 0).unwrap();
        chunk.write_bytecode(OpCode::Substract, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_division() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(6.0), 0).unwrap();
        chunk.add_constant(Value::Number(2.0), 0).unwrap();
        chunk.write_bytecode(OpCode::Divide, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_mult() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(3.0), 0).unwrap();
        chunk.add_constant(Value::Number(2.0), 0).unwrap();
        chunk.write_bytecode(OpCode::Multiply, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), Value::Number(6.0));
    }

    #[test]
    fn test_add_mult() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(1.0), 0).unwrap();
        chunk.add_constant(Value::Number(2.0), 0).unwrap();
        chunk.add_constant(Value::Number(3.0), 0).unwrap();
        chunk.write_bytecode(OpCode::Multiply, 0);
        chunk.write_bytecode(OpCode::Add, 0);
        chunk.write_bytecode(OpCode::Return, 0);
        let mut vm = Vm::new(chunk, false);
        assert_eq!(vm.run().unwrap(), Value::Number(7.0));
    }
}
