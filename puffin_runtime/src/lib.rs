pub mod chunk;
pub mod library;
pub mod op;
pub mod runtime;
pub mod value;
pub mod vm;
pub mod dom;
pub mod event;

use std::num::{ParseFloatError, ParseIntError};
pub use chunk::Chunk;

pub use ratatui;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ImageWidgetError(#[from] ratatui_image::errors::Errors),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    ParseColorError(#[from] ratatui::style::ParseColorError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),

    #[error("Unrecognized op code (0x{op:x}) at: 0x{pc:x}")]
    UnrecognizedOpCode { op: u8, pc: usize },

    #[error("Use of invalid op code at: 0x{pc:x}")]
    InvalidOpCode { pc: usize },

    #[error("Trying to access out of bounds memory (0x{at:x}) at: 0x{pc:x}")]
    AccessOutOfBounds { at: usize, pc: usize },

    #[error("Trying to access out of bounds values on the stack (0x{at:x}) at: 0x{pc:x}")]
    StackOutOfBounds { at: usize, pc: usize },

    #[error("Trying to access out of bounds constant (0x{at:x}) at: 0x{pc:x}")]
    InvalidConstantAccess { at: usize, pc: usize },

    #[error("Unable to perform binary {op} operation on {lhs_type} and {rhs_type}")]
    InvalidBinaryOperation {
        op: String,
        lhs_type: String,
        rhs_type: String,
    },

    #[error("Unable to perform unary {op} operation on {rhs_type}")]
    InvalidUnaryOperation { op: String, rhs_type: String },

    #[error("No field on object matching the name {name}")]
    NoFieldMatchingName { name: String },

    #[error("Attempt to divide by zero")]
    DivideByZero,

    #[error("Expected value on the stack but the stack was empty")]
    StackEmpty,

    #[error("Expected value on the intermediate stack but the stack was empty")]
    IntermediateStackEmpty,

    #[error("Global variable of name '{name}' not found")]
    GlobalNotFound { name: String },

    #[error("Expected {expected} got {ty}")]
    IncorrectType { ty: String, expected: String },

    #[error("Attempting to execute instructions with an empty call stack")]
    CallStackEmpty,

    #[error("Invalid assignment target ({ty})")]
    InvalidAssignmentTarget { ty: String },

    #[error("Method called for {name} without supplied `this` parameter")]
    MissingThisInMethodCall { name: String },

    #[error("Invalid hexadecimal string length. Got {got}, expected {expected} (not including '#' symbol)")]
    InvalidHexStringLength { got: usize, expected: usize },

    #[error("Invalid constraint ({name})")]
    InvalidConstraintName { name: String },

    #[error("Invalid constraint: {reason}")]
    InvalidConstraint { reason: String },
    
    #[error("Index {index} is out of bounds for list of size {size}")]
    IndexOutOfBounds { index: usize, size: usize }
}
