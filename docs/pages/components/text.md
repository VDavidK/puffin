# text

The `text` component takes in one argument as a child, converts it to a string and renders it
to the node tree.

## Properties

* `color: string` The foreground color of the text (using [color](/pages/props/colors.md) format)
* `bg: string` The background color of the text (using [color](/pages/props/colors.md) format)

## Example usage

```
let name = "Ben";

layout {
    // Renders '10' to the screen
    text 10;
    
    // Renders '30' to the screen
    text 10 + 20;
    
    // Renders 'Hello World' to the screen
    text "Hello World";
    
    // Renders 'Hello Ben!' to the screen. This value
    // will change if the `name` variable is updated
    text "Hello " + self.ben + "!";
}
```