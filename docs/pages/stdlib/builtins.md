# Built-in functions

## len

Returns an `integer` specifying the length of an iterable object.

**Parameters**

* `iterable: list | string | dictionary` The iterable.

**Returns**

The length of the iterable.

!> Only supports strings, lists and dictionaries. Passing any other type will crash the program.

## block

Creates a block element from a list of nodes.

**Parameters**

* `nodes: list[node]` A list of node elements.

**Returns**

A block element containing the nodes passed to the function.

## render

Creates a node from an instance for rendering purposes.

**Parameters**

* `instance: instance` An instance of a component.

**Returns**

A node element.

## clone

Creates a deep copy of a value.

**Parameters**

* `value: value` Any arbitrary value.

**Returns**

A deep copy of the passed value.

!> Instances are not made unique when cloned.