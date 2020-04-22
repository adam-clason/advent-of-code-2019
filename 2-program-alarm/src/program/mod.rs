mod memory;
mod operation;

use std::fs;

use memory::Memory;
use operation::OperationAction;

pub struct ProgramInput {
    pub noun: u32,
    pub verb: u32,
}

pub struct Program {
    memory: Memory,
}

impl Program {
    
    pub fn new_from_file(filename: &str) -> Result<Self, std::io::Error> {
        let memory_data = fs::read_to_string(filename)?;

        let memory_data: Vec<u32> = 
            memory_data.trim()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
    
        let program = Program {
            memory: Memory::new(memory_data)
        };
        
        Ok(program)
    }
    
    pub fn with_input(mut self, input: &ProgramInput) -> Program {
        self.memory.set_value(1, input.noun);
        self.memory.set_value(2, input.verb);
        self
    }
    
    pub fn run(&mut self) -> u32 {
        while let Some(operation) = self.memory.next() {
            let result_option = match operation.action {
                OperationAction::Add => Some(operation.param1 + operation.param2),
                OperationAction::Multiply => Some(operation.param1 * operation.param2),
                OperationAction::End => None
            };
              
            if let Some(result) = result_option {
                self.memory.set_value(operation.output_address, result);
            } else {
                break;
            }
        }
        
        self.memory.get_output()
    }
    
}
