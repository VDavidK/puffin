# string

## push

Appends a string to the end of another string.

**Parameters**

* `string: string` The initial string to modify.
* `value: string` A string to add to the other string.

**Returns**

The value appended to the string.

## push_front

Appends a string to the start of another string.

**Parameters**

* `string: string` The initial string to modify.
* `value: string` A string to add to the other string.

**Returns**

The value added to the string.

## pop

Removes a character from the end of a string.

**Parameters**

* `string: string` The initial string to modify.

**Returns**

The character removed from the string if it exists, or `null` if it does not.

## pop_front

Removes a character from the front of a string, shifting all subsequent characters to the left.

**Parameters**

* `string: string` The initial string to modify.

**Returns**

The character removed from the string if it exists, or `null` if it does not.

## replace

Replaces all occurrences of a specific string with another string.

**Parameters**

* `string: string` The initial string to modify.
* `from: string` The pattern to be replaced.
* `to: string` The pattern to replace `from` with.

**Returns**

A new string with the requested modifications made to it.

## insert

Inserts a string into another string, shifting all subsequent characters to the right.

**Parameters**

* `string: string` The initial string to modify.
* `value: string` A string to add to the other string.
* `idx: integer` The index to insert `value` at.

**Returns**

The value appended to the string.

?> Returns `null` if the index is out of bounds.

## remove

Removes a character from a string.

**Parameters**

* `string: string` The initial string to modify.
* `idx: integer` The index to remove from.

**Returns**

The character removed from the string if it exists, or `null` otherwise.

## contains

Returns a `boolean` indicating if a pattern is present in a string.

**Parameters**

* `string: string` The initial string to query.
* `pattern: string` The value to query for.

**Returns**

A `boolean` indicating if the pattern could be found in the string.

## to_lower

Creates a lower-case variant of a string.

**Parameters**

* `string: string` The initial string to modify.

**Returns**

A new string with all upper-case letters in the original string converted to lower-case.

## to_upper

Creates an upper-case variant of a string.

**Parameters**

* `string: string` The initial string to modify.

**Returns**

A new string with all lower-case letters in the original string converted to upper-case.

## split

Splits a string at every occurrence of a pattern, optionally limiting the number of splits.

**Parameters**

* `string: string` The initial string to modify.
* `pattern: string` The pattern to split at
* `n?: integer` The maximum number of elements contained in the resulting list.

**Returns**

A list of strings with each entry being a split off of the pattern. If the `n` argument is passed, the resulting list will be no longer than the value of `n`.

## substring

Returns a substring of another string.

**Parameters**

* `string: string` The initial string to modify.
* `idx: integer` The offset to start the substring from.
* `len: integer` The length of the substring.

**Returns**

A new string starting from `idx` of the original string with a length of `len`.

!> If the values of `idx` or `len` are negative, the program will crash.

!> If `idx` or the substring are out of bounds of the length of the string, the program will crash.