# list

The `list` module can be used to manipulate list objects.

## push

Adds a value to the back of the list.

### Parameters

* `list: list` The list to modify.
* `value: value` The value to push.

### Returns

The value pushed to the list.

## pop

Removes a value from the back of the list.

### Parameters

* `list: list` The list to modify.
* `value: value` The value to remove.

### Returns

The value removed from the list.

## push_front

Adds a value to the front of the list, shifting all other elements to the right.

### Parameters

* `list: list` The list to modify.
* `value: value` The value to push.

### Returns

The value pushed to the list.

## pop_front

Removes a value from the front of the list, shifting all other elements to the left.

### Parameters

* `list: list` The list to modify.
* `value: value` The value to remove.

### Returns

The value removed from the list.

## replace

Replaces a value in the list at a specified index.

### Parameters

* `list: list` The list to modify.
* `value: value` The value to replace with.
* `idx: integer` The index to replace at.

### Returns

The value used to replace at the specified index.

!> If the index is out of bounds, it will crash the program.

## insert

Inserts a value at the specified index of the list.
If an element is present at the index, it and all subsequent elements are shifted to the right.

### Parameters

* `list: list` The list to modify.
* `value: value` The value to insert.
* `idx: integer` The index to insert at.

### Returns

The value used to insert at the specified index.

!> If the index is out of bounds, it will crash the program with a single exception:
if the index and the length of the list are equal, in which case the list will grow to fit the element.

## remove

### Parameters

* `list: list` The list to modify.
* `idx: integer` The index to remove at.

Removes a value from the list at the specified index, shifting all subsequent elements to the left.
Returns the removed value.

### Returns

The value removed at the specified index.

!> If the index is out of bounds, it will crash the program.