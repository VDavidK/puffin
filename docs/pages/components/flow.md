# vbox & hbox

The `vbox` and `hbox` components are used to arrange their child components in a
certain manner. By default, each child component will take up as much space as it
can, however this can be modified using the `segments` property

The `vbox` component lays out its children in a vertical orientation while the `hbox` component lays them out horizontally.

## Properties

* `segments: list[string] | string` Specifies the size of each child component. If a string is used, then that size will be applied to all child components (uses the [size](/pages/props/sizes.md) string format)

## Example usage

```
layout {
    // Lays out its childs in a vertical where the first element only takes up one row
    vbox segments=["Length:1"] {
        text "Hello"; // Takes up one row
        text "World"; // Takes up the remaining space
    }
    
    // Lays out its children horizontally where every element is 30 columns wide
    hbox segments="Length:30" {
        text "Foo";
        text "Bar";
        text "Baz";
    }
}
```
