# Creating a simple application

This tutorial will cover how to create a simple image viewer using Puffin. This tutorial assumes you have read
through the previous sections under *Getting started*. Please read through those first if you have not done so already.

## Defining the state

To start with, let's create the application file and define the state needed. We will need the current user input as well as the final path to show.

```
// ImageViewer.puff

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
// ImageViewer.puff

@on_key
fn key_event_handler(event) {

    // If a regular key is pressed, add the corresponding character to the 'user_input' string
    if len(event.key) == 1 {
        self.user_input += event.key;
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
// ImageViewer.puff

fn submit_path() {
    // Set the file path to the final user input
    self.file_path = clone(self.user_input);
    
    // Clear out the user input
    self.user_input = "";
}
```

The next step is to define the user interface using the layout system. This uses the [vbox](/pages/components/flow.md) component
to lay out the components where the user input is at the top, taking only one row. The next row is the current image path
that is being displayed. Then the rest of the space is taken up by the image itself.

```
// ImageViewer.puff

layout {

    // The first two components will only take one row each, the remaining will take up
    // the rest of the space
    vbox segments=["Length:1", "Length:1"] {
    
        // User input indicator
        text "Enter the file path: " + self.user_input;
        
        // Shows the current file path
        text "Current file: " + self.file_path;
        
        if self.file_path != "" {
            image self.file_path;
        }
        
    }
    
}
```

This was the last piece needed for the application. It can now be run using the `puffin run ImageViewer.puff` command.

## Full source file

```
let user_input = "";
let file_path = "";

@on_key
fn key_event_handler(event) {

    // If a regular key is pressed, add the corresponding character to the 'user_input' string
    if len(event.key) == 1 {
        self.user_input += event.key;
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

fn submit_path() {
    // Set the file path to the final user input
    self.file_path = clone(self.user_input);

    // Clear out the user input
    self.user_input = "";
}

layout {

    // The first two components will only take one row each, the remaining will take up
    // the rest of the space
    vbox segments=["Length:1", "Length:1"] {

        // User input indicator
        text "Enter the file path: " + self.user_input;

        // Shows the current file path
        text "Current file: " + self.file_path;

        if self.file_path != "" {
            image self.file_path;
        }

    }

}
```