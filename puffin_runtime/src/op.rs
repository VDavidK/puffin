use num_enum::{IntoPrimitive, TryFromPrimitive};


#[derive(Debug, Clone, Copy, Default, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    Invalid = 0,

    // : literal [offset:4B]
    // Pushes a literal to the top of the stack
    Literal,

    // : print
    // Debug: Pops the top value off the stack and prints it
    Print,

    // ----------------------------
    // Arithmetic Instructions
    // ----------------------------

    // : add
    // Pops the top two values from the stack and pushes the summed result
    Add,

    // : sub
    // Pops the top two values from the stack and pushes the second minus the first
    Sub,

    // : mul
    // Pops the top two values from the stack and pushes the product result
    Mul,

    // : div
    // Pops the top two values from the stack and pushes the second divided by the first
    Div,

    // : mod
    // Pops the top two values from the stack and pushes the result
    Mod,


    // ----------------------------
    // Layout Codes
    // ----------------------------

    
}
