use crate::op::OpCode;

#[derive(Debug, Clone, Default)]
pub struct Program {
    code: Vec<OpCode>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}
