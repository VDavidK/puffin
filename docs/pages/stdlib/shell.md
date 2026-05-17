# shell

## exec

Executes a shell command.

**Parameters**

* `command: string` The command to execute.
* `arg*: value` A value which is a valid string.

**Returns**

A dictionary with two keys: `stdout` and `stderr` paired with strings, indicating the output of the command if it succeeded or failed respectively.

?> This function is variadic, meaning it can take any number of arguments.

!> It is not currently possible to elevate execution privileges, for example by using the sudo command.