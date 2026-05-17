# Creating custom blocks

Custom block nodes can be created using the `block` function. The primary use of this is to be
able to dynamically create elements to populate the node tree. This section assumes you have read
both the [Understanding components](/pages/advanced/understanding_components.md) and the [Using the render function](/pages/advanced/render_function.md) secitons
of the documentation.

The `block` function takes a list of nodes and converts it to a block node. This node can then be passed in as a
child of any node that accepts a block node as a child. For example, the `vbox`, `hbox` and `frame` elements.

```
let node = block([]);

new() {
    let text_instance = text("Hello", {});
    let text_node = render(text_instance);
    self.node = block([text_node]);
}

layout {
    // Both of these have the same effect
    vbox self.node;
    vbox {
        text "Hello";
    }
}
```

Below is an example of how you could use this to create a dynamic list of elements.

```
let text = ["Hello", "World"];
let node = block([]);

new() {
    let node_list = [];
    for i in 0:len(self.text) {
        let text_instance = text(self.text[i], {});
        let text_node = render(text_instance);
        list.push(node_list, text_node)
    }
    
    self.node = block(node_list);
}

layout {
    // Both of these have the same effect
    vbox self.node;
    vbox {
        text "Hello";
        text "World";
    }
    
    // However, the vbox above can be dynamically
    // updated with more or fewer elements at runtime.
}
```
