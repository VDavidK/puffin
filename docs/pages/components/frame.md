# frame

The `frame` component adds a border around its child components and otherwise behaves as
a regular `vbox` component. It does not have any properties.

## Example usage

```
layout {
    frame {
        // This text component has a frame around it
        text "Foo";
    }
    
    // This one, however, does not
    text "Bar";
}
```
