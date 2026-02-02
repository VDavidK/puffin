use num_enum::{IntoPrimitive, TryFromPrimitive};


#[derive(Debug, Clone, Copy, Default, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    Invalid = 0,

    // ----------------------------
    // Stack Instructions
    // ----------------------------

    // : literal [offset:4B]
    // Pushes a literal to the top of the stack
    Literal,

    // : getl [offset:4B]
    // Pushes the value of a local variable stored at the offset to the stack
    GetLocal,

    /// : setl [offset:4B]
    /// Overwrites the value of a local stored at the offset with the value at the top of the stack
    SetLocal,

    // : getg [offset:4B]
    // Pushes the value of a global variable matching the name of the literal at the given offset to the stack
    GetGlobal,

    // : setg [offset:4B]
    // Overwrites the value of a global variable matching the name of the literal at the given offset with the value at the top of the stack
    SetGlobal,

    // : pop
    // Pops the top value off the stack
    Pop,

    // ----------------------------
    // Object Manipulation Instructions
    // ----------------------------

    // : newobj
    // Pushes a new object to the top of the stack
    NewObject,

    // : setf
    // Sets the field of the object one below the top of the stack with the name matching the literal given to the value at the top of the stack
    SetField,

    // : getf
    // Pushes the value of the field of the object on the top of the stack with the name matching the literal given
    GetField,

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
    // Terminal Instructions
    // ----------------------------

    // : poll
    // Waits for user input and runs the corrisponding event handlers
    Poll,

    // : render
    // Renders the current layout buffer to the terminal
    Render,

    // ----------------------------
    // Layout Instructions
    // ----------------------------

    
}
