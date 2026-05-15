
```rust
error {
    SomeError,
    OtherError,
}

fn foo() {
    throw SomeError;
}

// Potential feature
fn handler(err) {
}

fn main() {
    let a = foo() catch {
        SomeError => 10,
        default => null,
    };

    // Potential feature
    foo() catch handler;

    foo() catch {
        SomeError => {
            // Do stuff
        },
        OtherError => return,
        default => raise,
    }
}
```