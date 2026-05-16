# Colors

Colors are defined as strings, where the string gets parsed into its respective color.
Both color names and hex codes are allowed and a list of valid names is shown below.
In order to use the default color, the `reset` color can be used.

## Valid color names

* `reset`
* `black`
* `red`
* `green`
* `yellow`
* `blue`
* `magenta`
* `cyan`
* `gray`
* `darkgray`
* `lightred`
* `lightgreen`
* `lightyellow`
* `lightblue`
* `lightmagenta`
* `lightcyan`
* `white`

## Code example

```
layout {
    text color="red" "Foo"; // This will be red
    text color="green" "Bar"; // This will be green
    text color="#0000ff" "Baz"; // This will be blue
}
```
