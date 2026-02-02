
pub mod layout;
pub mod vm;
pub mod op;
pub mod chunk;
pub mod value;

pub use chunk::Chunk;
pub use value::Value;

use vm::Vm;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Unrecognized op code (0x{op:x}) at: 0x{pc:x}")]
    UnrecognizedOpCode { op: u8, pc: usize },

    #[error("Use of invalid op code at: 0x{pc:x}")]
    InvalidOpCode { pc: usize },

    #[error("Trying to access out of bounds memory (0x{at:x}) at: 0x{pc:x}")]
    AccessOutOfBounds { at: usize, pc: usize },

    #[error("Trying to access out of bounds values on the stack (0x{at:x}) at: 0x{pc:x}")]
    StackOutOfBounds { at: usize, pc: usize },

    #[error("Trying to access out of bounds literal (0x{at:x}) at: 0x{pc:x}")]
    InvalidLiteralAccess { at: usize, pc: usize },

    #[error("Unable to perform binary {op} operation on {lhs_type} and {rhs_type}")]
    InvalidBinaryOperation { op: String, lhs_type: String, rhs_type: String },

    #[error("Unable to perform unary {op} operation on {rhs_type}")]
    InvalidUnaryOperation { op: String, rhs_type: String },

    #[error("Attempt to divide by zero")]
    DivideByZero,

    #[error("Expected value on the stack but the stack was empty")]
    StackEmpty,

    #[error("Global variable of name '{name}' not found")]
    GlobalNotFound { name: String },

    #[error("Expected '{expected}' got {ty}")]
    IncorrectType { ty: String, expected: String },
}


pub fn run(program: &Chunk) -> Result<(), RuntimeError> {
    let mut vm = Vm::new(program);

    while vm.is_running() {
        vm.execute()?;
    }

    Ok(())
}
