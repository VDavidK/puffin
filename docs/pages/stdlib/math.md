# math

## pow

Returns the provided value to the power of the other provided value.

**Parameters**

* `value: integer | float` The initial value.
* `exponent: integer | float` The exponent for the product.

**Returns**

$value^{exponent}$.

## cos

Returns the cosine of a number.

**Parameters**

* `value: integer | float` The initial value.

**Returns**

$cos(value)$.

## sin

Returns the sine of a number.

**Parameters**

* `value: integer | float` The initial value.

**Returns**

$sin(value)$.

## tan

Returns the tangent of a number.

**Parameters**

* `value: integer | float` The initial value.

**Returns**

$tan(value)$.

## acos

Returns the arc cosine of a number.

**Parameters**

* `value: integer | float` The initial value.

**Returns**

$acos(value)$ (also written as $cos^{-1}(value)$).

## asin

Returns the arc sine of a number.

**Parameters**

* `value: integer | float` The initial value.

**Returns**

$asin(value)$ (also written as $sin^{-1}(value)$).

## atan

Returns the arc tangent of a number.

**Parameters**

* `value: integer | float` The initial value.

**Returns**

$atan(value)$ (also written as $tan^{-1}(value)$).

## to_rad

Converts degrees to radians.

**Parameters**

* `value: integer | float` Degrees.

**Returns**

A numeric radians value.

## to_deg

Converts radians to degrees.

**Parameters**

* `value: integer | float` Radians.

**Returns**

A numeric degrees value.

## max

Returns the largest numeric value in a series of values.

**Parameters**

* `value*: integer | float` A numeric value.

**Returns**

The largest number passed to the function.

?> This function is variadic, meaning it can take any number of arguments.
!> If a non-numeric value is passed to the function, the program will crash.

## min

Returns the smallest numeric value in a series of values.

**Parameters**

* `value*: integer | float` A numeric value.

**Returns**

The smallest number passed to the function.

?> This function is variadic, meaning it can take any number of arguments.
!> If a non-numeric value is passed to the function, the program will crash.

## clamp

Clamps a number between two other numbers, limiting itself to a number in that range.

**Parameters**

* `value: integer | float` A numeric value.
* `min: integer | float` The minimum value for `value`.
* `max: integer | float` The maximum value for `value`.

**Returns**

The value of `value` if it is between `min` and `max`, `min` if it is greater than `value` or `max` if it is lesser than `value`.
