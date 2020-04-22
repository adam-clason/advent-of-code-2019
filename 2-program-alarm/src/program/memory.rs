use super::operation::{Operation, OperationAction};

pub struct Memory {
    index: usize,
    memory: Vec<u32>,
}

impl Memory {  
    pub fn new(values: Vec<u32>) -> Self {
        Memory {
            index: 0,
            memory: values,
        }
    }
    
    pub fn dereference(&self, address: usize) -> u32 {
        let ref_address = self.retrieve(address);
        self.memory[ref_address as usize]
    }
    
    pub fn retrieve(&self, address: usize) -> u32 {
        self.memory[address]
    }
    
    pub fn set_value(&mut self, address: usize, value: u32) {
        self.memory[address] = value;
    }
    
    pub fn get_output(&self) -> u32 {
        self.memory[0]
    }
}

impl Iterator for Memory {
    type Item = Operation;
    
    fn next(&mut self) -> Option<Operation> {
        let op_code = self.retrieve(self.index);
        let op_action = OperationAction::from_op_code(op_code);
        
        let operation_option = match op_action {
            OperationAction::End => None,
            _ =>   Some(Operation {
                            action: op_action,
                            param1: self.dereference(self.index + 1),
                            param2: self.dereference(self.index + 2),
                            output_address: self.retrieve(self.index + 3) as usize
                    }),
        };
     
        self.index += 4;
        
        operation_option
    }
}