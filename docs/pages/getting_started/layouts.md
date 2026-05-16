# Layouts

Layouts provide a means of piecing together components to create a completed application.
The components will then be rendered in the order given.

The *Built-in components* chapter of the documentation has a list of all available components that
can be used when putting together a layout.

Layouts are made using the `layout` keyword followed by a matching pair of braces. Between those braces a list of components can be given.

## Component values

Components defined in layouts have to be passed in a value. This value can either be a block component (denoted with a matching pair of
braces) or a literal value (e.g. 10, "hello", etc.).

```
// Defines a layout with a single text component writing "Hello World" to the terminal.
layout {
    text "Hello World";
}
```

```
// Defines a layout two text components inside of a vbox component (using a block component)
layout {
    vbox {
        text "Hello";
        text "Hello";
    }
}
```

Components within a layout will always try to take up as much space as possible. This means that in the example above,
there will be a lot of space between the text as each of those components take up half of the terminal screen.

## Properties

Components can receive optional properties with the `prop=value` syntax. Each component takes in a different selection of properties
which can be referenced in the *Built-in components* section of the documentation.

```
// Defines a layout with a single text component that writes "Hello World" in red to the terminal.
layout {
    text color="red" "Hello World";
}
```

## Control flow

Control flow such as `if`, `match` and `for` are available inside of layouts. These can be used to conditionally render
components.

### If statements

If statements follow the structure of `if condition { components }`. Both `else` and `else if` can follow the closing parenthesis to add extra steps in case the condition evaluates to false.

```
layout {
    // Basic if statement
    if 1 + 2 == 3 {
        text "This is true!";
    }
    
    // Using 'else' with an if statement
    if 2 + 3 == 4 {
        text "This is not true, so you won't see this";
    } else {
        text "Hello!";
    }
    
    // If statements can be chained together with 'else if'
    if 1 == 2 {
        text "Foo";
    } else if 2 == 3 {
        text "Bar";
    } else {
        text "Baz";
    }
}
```

### Match statements

For long if/else chains, the match statement might be a better solution. To use a match statement
you can use the `match` keyword followed by an expression then a matching pair of braces. Inside
these braces, conditions can be added by putting an expression followed by the `=>` arrow and another
pair of matching braces. Components can then be added inside those braces which will only be shown
when both expressions evaluate to the same value.

Default cases can be added with the `default` keyword instead of an expression. The components in
the default case will get shown if no other case is picked.

Additionally, an identifier can follow the `default` keyword to capture the value that the match case received.

```
layout {
    match 10 {
        10 => {
            text "The number is 10.";
        }
        20 => {
            text "The number is 20!";
        }
        default => {
            text "The number is something else";
        }
    }
    
    match 10 {
        // default can capture the value passed in
        default value => {
            text "The number is " + value;
        }
    }
}
```

### For loops

!> The `for` loop is considered unstable and should not generally be used. This can be worked around using the techniques shown in the *Advanced layouts* section of the documentation.

For loops are defined with the `for` keyword followed by a variable name, the `in` keyword, a range in the format of `from:to` then finally a matching pair of braces.
The for loop will then iterate over the range (the range being exclusive), generating the components each iteration.

The variable defined in the for loop statement is accessible within the for loop body.

```
layout {
    for i in 0:5 {
        text i;
    }
    
    // Will result in the following:
    text 0;
    text 1;
    text 2;
    text 3;
    text 4;
}
```
