# Event handling

Events such as key presses and mouse clicks can be listened to using the decorator syntax. Functions declared with an
event decorator are called handler functions and cannot be called directly and its name is not used. Handler functions
are typically passed an event object with parameters related to the event they received.

```
@on_key
fn key_handler(event) {
    // Exit if the user presses the 'q' key
    if event.key == "q" {
        exit();
    }
}
```

## List of all available events

* on_key
  * key: string
  * modifiers: list[string]

* on_button
  * button: string
  * column: int
  * row: int
  * modifiers: list[string]

* on_focus
* on_unfocus
* on_paste
  * content: string

* on_resize
  * columns: int
  * rows: int
