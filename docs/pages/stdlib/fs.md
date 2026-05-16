# fs

## write

Writes a string to a file, creating it if it does not exist.

### Parameters

* `path: string` Specifies the relative path to the target file.
* `content: value` The content to write to the file.
* `append: boolean | null` Whether to append to the file or truncate it before writing.

?> The function is currently not variadic.
If you wish to truncate the file before writing, `false` or `null` as the third parameter.