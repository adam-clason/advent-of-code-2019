pub enum OperationAction {
    Add,
    Multiply,
    End,
}

pub struct Operation {
    pub action: OperationAction,
    pub param1: u32,
    pub param2: u32,
    pub output_address: usize,
}

impl OperationAction {
    pub fn from_op_code(op_code: u32) -> Self {
        match op_code {
            1 => Self::Add,
            2 => Self::Multiply,
            _ => Self::End,
        }
    }
}

