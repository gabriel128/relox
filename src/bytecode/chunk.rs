use std::usize;
use crate::{Result, errors::ReloxError};

use super::value::Value;

/// Op Codes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    Constant { constant_offset: u8 },
    Negate,
    Return,
    Add,
    Substract,
    Divide,
    Multiply,
    Nil,
    True,
    False
}
const CONSTANT_POOL_MAX: usize = 255;

/// Chunk
///
/// Represents a chunk of Opcodes. It can be thought as an array of bytes
#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    constant_pool: Vec<Value>,
    lines: Vec<u16>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constant_pool: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn instruction_at(&self, index: usize) -> Option<&OpCode> {
        self.code.get(index)
    }

    pub fn line_at(&self, index: usize) -> u16 {
        *self.lines.get(index).unwrap_or(&0)
    }

    pub fn read_constant(&self, index: u8) -> Option<&Value> {
        self.constant_pool.get(index as usize)
    }

    pub fn write_bytecode(&mut self, op_code: OpCode, line: u16) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, constant: Value, line: u16) -> Result<()> {
        if self.constant_pool.len() >= CONSTANT_POOL_MAX {
            return ReloxError::new_fatal_error("Constant Pool max reached".to_string());
        }

        self.constant_pool.push(constant);
        self.write_bytecode(
            OpCode::Constant {
                constant_offset: (self.constant_pool.len() - 1) as u8,
            },
            line,
        );
        Ok(())
    }

    pub fn dissasemble(&self) {
        println!("== Dissasembling Chunk ==");
        println!("byte_offset   lines   op    data");
        println!("");

        let mut byte_offset = 0;

        for (i, opcode) in self.code.iter().enumerate() {
            self.dissasemble_instruction(opcode, i, &mut byte_offset)
        }
    }

    pub fn dissasemble_instruction(&self, opcode: &OpCode, i: usize, byte_offset: &mut usize) {
        let extra_chunk = match opcode {
            OpCode::Constant { constant_offset } => {
                format!("{:?}", self.constant_pool[*constant_offset as usize])
            }
            _ => "".to_string(),
        };
        println!(
            "{:?}             {:?}      {:?}    {}",
            byte_offset, self.lines[i], opcode, extra_chunk
        );
        *byte_offset += std::mem::size_of_val(opcode);
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of_val;

    use super::*;

    #[test]
    fn test_chunk() {
        let mut chunk = Chunk::new();
        chunk.add_constant(Value::Number(3.0), 22).unwrap();
        chunk.add_constant(Value::Number(4.0), 22).unwrap();
        chunk.write_bytecode(OpCode::Add, 22);
        chunk.write_bytecode(OpCode::Return, 23);

        println!("Size of bool {:?}", size_of_val(&Value::Number(3.3)));
        println!("Size of Value {:?}", size_of_val(&Value::Number(3.3)));
        println!("Size of OpCode {:?}", size_of_val(&OpCode::Return));
        println!("Chunk size {:?}", size_of_val(&chunk));
        println!("Chunk instr code size {:?}", size_of_val(&chunk.code));
        println!("Constant pool size {:?}", size_of_val(&chunk.constant_pool));
        println!("Chink lines size {:?}", size_of_val(&chunk.lines));
        // chunk.dissasemble();
    }
}
