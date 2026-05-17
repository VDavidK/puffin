# dict

The `dict` module can be used to manipulate dictionary objects.

## insert

Inserts a key and a value into the dictionary, overriding the prior value associated with the key if it is present.

**Parameters**

* `dict: dictionary` The dictionary to modify.
* `key: value` The key to add to the dictionary.
* `value: value` The value associated with the key.

**Returns**

The value passed to the function.

## contains

Returns a `boolean` indicating whether a key is present in the dictionary.

**Parameters**

* `dict: dictionary` The dictionary to modify.
* `key: value` The key to query for.

**Returns**

A `boolean` indicating whether a key is present in the dictionary.

## remove

Removes an entry from the dictionary based on its key, returning the value associated with it.

**Parameters**

* `dict: dictionary` The dictionary to modify.
* `key: value` The key to remove from the dictionary.

**Returns**

The value if it was present in the dictionary, otherwise `null`.

## keys

Returns a list containing all of the keys in the dictionary.

**Parameters**

* `dict: dictionary` The dictionary to modify.

**Returns**

A one-dimensional list of the keys present in the dictionary.

?> The keys are not guaranteed to be in any specific order.

## values

Returns a list containing all of the values in the dictionary.

**Parameters**

* `dict: dictionary` The dictionary to modify.

**Returns**

A one-dimensional list of the values present in the dictionary.

?> The values are not guaranteed to be in any specific order.

## entries

Returns a two-dimensional list containing all of the pairs in the dictionary.

**Parameters**

* `dict: dictionary` The dictionary to modify.

**Returns**

A two-dimensional list containing all of the pairs in the dictionary. The first value in each nested list is the key, while the second is the value.

?> The pairs are not guaranteed to be in any specific order.