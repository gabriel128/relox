use std::usize;

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
}

pub type Value = f64;
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

    pub fn read_constant(&self, index: u8) -> Option<&Value> {
        self.constant_pool.get(index as usize)
    }

    pub fn write_bytecode(&mut self, op_code: OpCode, line: u16) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, constant: Value, line: u16) -> Result<(), String> {
        if self.constant_pool.len() >= 255 {
            return Err("Constant Pool max reached".to_string());
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
            OpCode::Return
            | OpCode::Negate
            | OpCode::Add
            | OpCode::Substract
            | OpCode::Divide
            | OpCode::Multiply => "".to_string(),
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
    // use std::mem::size_of_val;

    // use super::*;

    #[test]
    fn test_chunk() {
        // let mut vec = Vec::<OpCode>::new();

        // println!("Empty Vec {:?}", size_of_val(&vec));
        // println!("Size of OpCode {:?}", size_of_val(&OpCode::Return));
        // vec.push(OpCode::Return);
        // vec.push(OpCode::Constant { constant_offset: 2 });
        // println!("Size of Vec with return {:?}", size_of_val(&*vec));

        // let mut chunk = Chunk::new();
        // chunk.add_constant(3.0, 22).unwrap();
        // chunk.add_constant(4.0, 22).unwrap();
        // chunk.write_bytecode(OpCode::Return, 23);
        // chunk.dissasemble();
    }
}
