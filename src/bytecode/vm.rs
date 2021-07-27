use crate::bytecode::chunk::OpCode;

use super::chunk::Chunk;

#[derive(Debug)]
struct Vm {
    chunk: Chunk,
    ip: usize,
    debug_mode: bool
}

enum VmError {
    CompileError,
    RuntimeError
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 1, debug_mode: false }
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
                    },
                    OpCode::Return => Ok(()),
                };

                return res;
            } else {
                return Err(VmError::CompileError);
            }

        }
    }
}
