# Custom components

Every puffin file is treated as its own component. These components allow for separating large projects into multiple files.
The name of the file will be picked as the name of the component, so a file named 'CustomInput.puff' will become a component
named 'CustomInput'.

Files can contain a constructor by using the `new` keyword. Constructors are only called once when the component is created.
Since the main file's component is only created once, the main constructor can be used for initialization logic.

```
new() {
    // Initialization logic
    shell.exec("setup.sh");
}
```

Constructors take two arguments, `children` and `properties`. These are the same parameters that are passed in the layouts
(e.g. `CustomComponent prop=10 "Inner";` has the values `"Inner"` as the `children` parameter and the dictionary `{ prop: 10 }` as the `properties` parameter)

```
let value = null;

new(children, props) {
    self.value = props.some_prop;
}
```
