# Understanding components

Components in puffin are actually classes. When adding components to the layout, an instance of the
component's class is created and bound to a component node. This node is then added to the node tree where
the node tree stores the hierarchy of all the components in the application for rendering and event dispatching purposes.

Because components are classes, they can be manually created inside of regular functions. They always require two arguments,
`children` and `properties`. `children` can be any value, but `properties` has to be a dictionary. These are equivalent
of the inner value and properties that are given in the layouts using the `prop=value` syntax as well as `component inner;`

```
fn foo() {
    let text_instance = text("Hello World", {});
}
```

This text instance cannot be used directly inside of layouts as they are instances and not nodes, however they can be converted
into nodes as shown in the [Using the render function](/pages/advanced/render_function.md) section of the documentation.
