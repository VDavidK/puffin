# Basic syntax

## Variables

Variables are declared using the `let` keyword. When declared inside of function bodies, they are scope bound.
However, if declared outside of function bodies then they are bound to the file's component.

Uninitialized variables are not supported where variables always need to be initialized.

```
// Creates a new variable bound to the file's component
let a = 0;

fn example() {
    // Creates a new variable on the stack
    let b = 10;
    
    // Component variables are accessed through the 'self' keyword
    if self.a == b {
        // Creates a new variable within the if statement's scope
        let c = 20;
    }
    
    // 'c' falls out of scope and is no longer accessible.
}
```

## Using the 'self' keyword

When accessing variables and functions defined in the top scope of a file (the component scope). The `self` keyword
is necessary.

```
let a = 10;

fn foo() {
    // Invalid: 'a' is not defined in the current scope.
    a = 20;
    
    // Valid
    self.a = 20;
    
    // 'self' is also required for function calls
    self.bar();
}

fn bar() {
    // ...
}
```

## Simple FizzBuzz example

```
fn example() {
    // Define variables with ‘let
    let value = 7;
    
    // Call functions like normal
    let result = fizzbuzz(value);
}

// Calculates the FizzBuzz result of a value
fn fizzbuzz(value) {
    if value % 15 == 0 {
        return "FizzBuzz";
    } else if value % 5 == 0 {
        return "Buzz";
    } else if value % 3 == 0 {
        return "Fizz";
    }
    
    // Cast the number to a string
    return str(value)
}
```
