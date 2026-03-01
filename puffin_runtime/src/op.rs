use num_enum::{IntoPrimitive, TryFromPrimitive};


#[derive(Debug, Clone, Copy, Default, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    Invalid = 0,

    // ----------------------------
    // Stack Instructions
    // ----------------------------

    // : const [offset:4B]
    // Pushes a constant to the top of the stack
    Constant,

    // : pop
    // Pops the top value off the stack
    Pop,

    // : getl [offset:4B]
    // Pushes the value of a local variable stored at the offset to the stack
    GetLocal,

    // : setl [offset:4B]
    // Overwrites the value of a local stored at the offset with the value at the top of the stack
    SetLocal,

    // : getg [offset:4B]
    // Pushes the value of a global variable matching the name of the constant at the given offset to the stack
    GetGlobal,

    // : setg [offset:4B]
    // Overwrites the value of a global variable matching the name of the constant at the given offset with the value at the top of the stack
    SetGlobal,

    // ----------------------------
    // Object Manipulation Instructions
    // ----------------------------

    // : newobj
    // Pushes a new object to the top of the stack
    NewObject,

    // : setf [offset:4B]
    // Pops the top two values off the stack. Assigns the field with the given name at the offset of the latter value to the first value
    SetField,

    // : getf [offset:4B]
    // Pops the top of the stack and pushes the value of the field with the given name at the offset to the stack
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

    // : neg
    // Pops the top value off the stack and pushes the negated value
    Neg,

    // : not
    // Pops the top value off the stack and pushes the negated truthy value
    Not,
    
    // : eq
    // Pops the top two values off and pushes true if they're equal and false if they're not.
    Eq,
    
    // : neq
    // Pops the top two values off and pushes true if they're not equal and false if they're not.
    Neq,

    // : ge
    // Pops the top two values off and pushes true if second is greater than or equal to the first and false if they're not.
    Ge,
    
    // : le
    // Pops the top two values off and pushes true if second is less than or equal to the first and false if they're not.
    Le,
    
    // : gt
    // Pops the top two values off and pushes true if second is greater than to the first and false if they're not.
    Gt,
    
    // : lt
    // Pops the top two values off and pushes true if second is less than to the first and false if they're not.
    Lt,


    // ----------------------------
    // Branch Instructions
    // ----------------------------
    
    // : jmp [addr:8B]
    // Sets the program counter to the specified address
    Jump,

    // : jmpi [addr:8B]
    // Pops the top of the stack and if the value is truthy, then sets the program counter to the specified address
    JumpIf,

    // : call
    // Pops the top function off the stack and calls it
    // Takes ownership of the arguments passed in.
    Call,

    // : ret
    // Pops the top value from the call stack and sets the program counter to that value. If the call stack is empty, then it exits the program
    // Pops all other local variables defined in the current scope
    Return,

    
    // ----------------------------
    // Terminal Instructions
    // ----------------------------

    // : exit
    // Exits the application
    Exit,

    // : poll
    // Waits for user input and runs the corresponding event handlers
    Poll,

    // : render
    // Renders the current layout buffer to the terminal
    Render,

    // ----------------------------
    // Layout Instructions
    // ----------------------------

    
}
