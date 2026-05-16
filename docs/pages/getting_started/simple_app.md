# Creating a simple application

This tutorial will cover how to create a simple image viewer using Puffin. This tutorial assumes you have read
through the previous sections under *Getting started*. Please read through those first if you have not done so already.

## Defining the state

To start with, let's define the state of the application. We will need the current user input as well as the final path to show.

```
let user_input = "";
let file_path = "";
```

The `user_input` variable will keep track of what the user has typed in already. This will then be used to update the `file_path`
variable which is used by the [image](/pages/components/image.md) component to display the actual image.

## Creating the key event handler

This handler will be responsible for allowing the user to type the image path and submit it to the image component.
In order to do so, the `@on_key` decorator is used. The handler will then have to check the received event's `key` property
to determine whether to add to, remove from or submit the text input.

Removing characters from a string is done by using the `pop` method in the `string` library.

?> It is always recommended to add a means of exiting the application. For example if the user presses the escape key, the application closes. This is done with the `exit()` [built-in function](/pages/stdlib/builtins.md)

```
@on_key
fn key_event_handler(event) {

    // If a regular key is pressed, add the corrisponding character to the 'user_input' string
    if len(event.key) == 1 {
        self.user_input += 1;
    }
    
    // If the backspace key was pressed, remove the last character from the 'user_input' string
    if event.key == "backspace" {
        string.pop(self.user_input);
    }
    
    // If the return key was pressed, submit the string to the image component
    if event.key == "enter" {
        self.submit_path();
    }
    
    // If the user presses the escape key, the application exits
    if event.key == "escape" {
        exit();
    }
    
}
```

The `self.submit_path()` function is not yet defined, so let's make it now.

This function's job is to update the `file_path` variable to the new string created. Because strings are mutable, the
`clone` (part of the [built-in functions](/pages/stdlib/builtins.md)) must be used to make a new copy of the input string.

```
fn submit_path() {
    self.file_path = clone(self.user_input);
}
```

The next step is to define the user interface using the layout system.

```
layout {
    vbox segments="Length:1" {
        text "Enter the file path: " + self.user_input;
        text "Current file: " + self.file_path;
    }
}
```
