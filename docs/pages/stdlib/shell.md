# shell

## exec

Executes a shell command.

### Parameters

* `command: string` The command to execute.
* `arg*: value` A value which is a valid string.

### Returns
A dictionary with two keys: `stdout` and `stderr` paired with strings, indicating the output of the command if it succeeded or failed respectively.

?> This function is variadic, meaning it can take any number of arguments.
!> Sudo commands do not work properly.