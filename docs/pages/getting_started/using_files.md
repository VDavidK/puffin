# Using files

## The `use` keyword

As many developers prefer segmenting their code into multiple files, Puffin offers them to do precisely that.

The `use` keyword imports another Puffin component from another `.puff` file.

````
// MyComponent.puff
use MyOtherComponent;

layout {
    MyOtherComponent {
    }
}
````

For nested directories, one can use dot notation.
In the following example, the component `MyOtherComponent` is imported from the path `first/second/MyOtherComponent.puff`.

?> Do note that all paths are relative to the root component.
````
// MyComponent.puff
use first.second.MyOtherComponent;

layout {
    MyOtherComponent {
    }
}
````