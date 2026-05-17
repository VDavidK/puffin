# Using the render function

The render function takes a component instance and converts it into a node to be used in the
node tree. In order to create component instances, see the [Understanding components](/pages/advanced/understanding_components.md) section of the documentation.

Nodes cannot be directly placed into the layout and must be passed as props to other components. There are no components that can
directly take in a component node, however the `block` function can be used to create nodes that can. See the [Creating custom blocks](/pages/advanced/block_function.md) section for more details.

```
fn foo() {
    let text_instance = text("Hello", {});
    let text_node = render(text_instance);
}
```
