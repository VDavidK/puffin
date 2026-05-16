## Types

### Primitive Types

Puffin features four primitive types. These are `boolean`, `integer`, `float` and `null`.

#### Boolean

A boolean is a value with two possible states: `true` and `false`.
These are invaluable for control-flow statements.

The following is an example of a function which returns `true` if `value` is greater than 2, but returns `false` if it is not.

````
fn is_greater_than_2(value) {
    return value > 2;
}
````

#### Integer

An integer is any non-decimal number (e.g. 100).

````
fn add_two_together(a, b) {
    return a + b;
}

fn foo() {
    // value = 3
    let value = add_two_together(1, 2);
}
````

?> Integers in Puffin are signed and 64 bits by default.

#### Float

A float or floating-point number is a decimal number (e.g. 100.5).

````
fn add_two_together(a, b) {
    return a + b;
}

fn foo() {
    // value_a = 3.58
    let value_a = add_two_together(1.25, 2.33);
    // value_b = 4.0
    let value_b = add_two_together(1.5, 2.5);
}
````

?> Floats in Puffin are 64 bits by default.

?> Any operation on a floating-point value with an integer will produce a
floating-point value, even if there is no decimal point in the result.

#### Null

`null` is the absence of a value.

All functions which do not return a value automatically return `null` when called.

?> `null` cannot be operated on with mathematical operators.

### Complex Types

Puffin features three complex types. These are `string`, `dictionary` and `list`.

#### String

A string is a mutable series of characters.

````
let my_string = "Hello, World!";

layout {
    // Displays the value of 'my_string' ("Hello, World!")
    text self.my_string;
}
````

?> Puffin supports UTF-8 characters.

#### Dictionary

A dictionary (also known as a hash map or hash table) can be used to map a key to a value.

The value associated with a key can be obtained by indexing the dictionary with said key.

````
let translation_table = {
    Hello: "Halló",
    Goodbye: "Bless",
}

layout {
    text self.translation_table["Hello"];
    text self.translation_table["Goodbye"];
}
````

?> Dictionaries return `null` if the entry does not exist rather than crashing the program.

?> Puffin currently only supports initializing dictionaries with identifiers as keys.
This can be worked around using the standard library's `dictionary.insert()`.

#### List

A list is a dynamic-sized array which can be used to store a sequence of values for iterative purposes.

To obtain the `nth` element of a list, index it with the value of `n`.

````
let my_list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn get_third() {
    // Returns 2
    return self.my_list[2];
}
````

?> List indices start at 0.

?> Lists return `null` if the index is out of bounds rather than crashing the program.