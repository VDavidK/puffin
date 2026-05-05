use num_enum::{IntoPrimitive, TryFromPrimitive};


#[derive(Debug, Clone, Copy, Default, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    Invalid = 0x00,

    // ----------------------------
    // Stack Instructions
    // ----------------------------

    // : const [offset:4B]
    // Pushes a constant to the top of the stack
    Constant = 0x10,

    // : pop
    // Pops the top value off the stack
    // Expected stack: TOP > [value: any]
    Pop = 0x11,

    // : getl [offset:4B]
    // Pushes the value of a local variable stored at the offset to the stack
    GetLocal = 0x12,

    // : setl [offset:4B]
    // Overwrites the value of a local stored at the offset with the value at the top of the stack
    // Expected stack: TOP > [value: any]
    SetLocal = 0x13,

    // : getg [offset:4B]
    // Pushes the value of a global variable matching the name of the constant at the given offset to the stack
    GetGlobal = 0x14,

    // : setg [offset:4B]
    // Overwrites the value of a global variable matching the name of the constant at the given offset with the value at the top of the stack
    // Expected stack: TOP > [value: any]
    SetGlobal = 0x15,

    // ----------------------------
    // Object Creation Instructions
    // ----------------------------

    // : class [offset:4B]
    // Pushes a new class to the top of the stack with the specified name
    NewClass = 0x20,

    // : newlist
    // Pushes a new list to the top of the stack
    NewList = 0x21,

    // : newdict
    // Pushes a new dictionary to the top of the stack
    NewDictionary = 0x22,

    // ----------------------------
    // Object Manipulation Instructions
    // ----------------------------

    // : scons
    // Expected stack: TOP > [value: fn] > [cls: class]
    SetConstructor = 0x30,

    // : getf [offset:4B]
    // Pops the top of the stack and pushes the value of the field with the given name at the offset to the stack
    // Expected stack: TOP > [obj: assignable]
    GetField = 0x31,

    // : setf [offset:4B]
    // Pops the top two values off the stack. Assigns the field with the given name at the offset of the latter value to the first value
    // Expected stack: TOP > [value: any] > [obj: assignable]
    SetField = 0x32,

    // : gidx []
    // Pops the top value of the stack, indexes the new top of the stack if it is indexable with the prior top value
    // Expected stack: TOP > [value: any] > [obj: indexable]
    GetIndex = 0x33,

    // TODO: Reserved slot (0x34) for SetIndex

    // : pusharr
    // Takes the top value form the stack and pushes it to the end of the list.
    // Expected stack: TOP > [value: any] > [lst: list]
    PushList = 0x35,

    // : poparr
    // Pops the top value from the list at the top of the stack and pushes it to the top of the stack.
    // Expected stack: TOP > [lst: list]
    PopList = 0x36,

    // : setmet [offset:4B]
    // Pops the top function off the stack and assigns it as a method to a class with the name of the constant at the provided offset.
    // Expected stack: TOP > [method: callable] > [cls: class]
    SetClassMethod = 0x37,

    // : sethand [offset:4B]
    // Pops the top function off the stack and assigns it as an event handler to a class with the name of the constant at the provided offset.
    // Expected stack: TOP > [handler: callable] > [cls: class]
    SetHandler = 0x38,

    // : reactive
    // Pops the top value of the stack and wraps it as a reactive value. Pushing the result back onto the stack.
    MakeReactive = 0x39,

    // ----------------------------
    // Arithmetic Instructions
    // ----------------------------

    // : add
    // Pops the top two values from the stack and pushes the summed result
    // Expected stack: TOP > [b] > [a]
    Add = 0x60,

    // : sub
    // Pops the top two values from the stack and pushes the second minus the first
    // Expected stack: TOP > [b] > [a]
    Sub = 0x61,

    // : mul
    // Pops the top two values from the stack and pushes the product result
    // Expected stack: TOP > [b] > [a]
    Mul = 0x62,

    // : div
    // Pops the top two values from the stack and pushes the second divided by the first
    // Expected stack: TOP > [b] > [a]
    Div = 0x63,

    // : mod
    // Pops the top two values from the stack and pushes the result
    // Expected stack: TOP > [b] > [a]
    Mod = 0x64,

    // : neg
    // Pops the top value off the stack and pushes the negated value
    // Expected stack: TOP > [a]
    Neg = 0x65,

    // : not
    // Pops the top value off the stack and pushes the negated truthy value
    // Expected stack: TOP > [a]
    Not = 0x66,
    
    // : eq
    // Pops the top two values off and pushes true if they're equal and false if they're not.
    // Expected stack: TOP > [a] > [b]
    Eq = 0x67,
    
    // : neq
    // Pops the top two values off and pushes true if they're not equal and false if they're not.
    // Expected stack: TOP > [a] > [b]
    Neq = 0x68,

    // : ge
    // Pops the top two values off and pushes true if second is greater than or equal to the first and false if they're not.
    // Expected stack: TOP > [a] > [b]
    Ge = 0x69,
    
    // : le
    // Pops the top two values off and pushes true if second is less than or equal to the first and false if they're not.
    // Expected stack: TOP > [a] > [b]
    Le = 0x6A,
    
    // : gt
    // Pops the top two values off and pushes true if second is greater than to the first and false if they're not.
    // Expected stack: TOP > [a] > [b]
    Gt = 0x6B,
    
    // : lt
    // Pops the top two values off and pushes true if second is less than to the first and false if they're not.
    // Expected stack: TOP > [a] > [b]
    Lt = 0x6C,


    // ----------------------------
    // Branch Instructions
    // ----------------------------
    
    // : jmp [addr:8B]
    // Sets the program counter to the specified address
    Jump = 0x80,

    // : jmpi [addr:8B]
    // Pops the top of the stack and if the value is truthy, then sets the program counter to the specified address
    // Expected stack: TOP > [val: any]
    JumpIf = 0x81,

    // : call [arity:1B]
    // Pops the top function off the stack and calls it
    // Takes ownership of the arguments passed in.
    // Expected stack: TOP > [val: function]
    Call = 0x82,

    // : ret
    // Pops the top value from the call stack and sets the program counter to that value. If the call stack is empty, then it exits the program
    // Pops all other local variables defined in the current scope
    // Expected stack: TOP > [return_val: any] > ...
    Return = 0x83,

}
