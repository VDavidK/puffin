use num_enum::FromPrimitive;


#[derive(Debug, Clone, Copy, Default, FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    Invalid = 0,

    // : push [literal]
    // Pushes a single literal to the top of the stack
    PushLiteral,

    // : add
    // Pops the top two values from the stack and pushes the result
    Add,
}
