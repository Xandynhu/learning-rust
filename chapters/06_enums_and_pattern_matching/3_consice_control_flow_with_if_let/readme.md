# Consice Control Flow With `if let`

In Rust, we can use `if let` to write concise control flow that matches a single pattern. This is useful when we want to match a single pattern and ignore the rest.

```rust
fn main() {
    let some_value = Some(5);

    if let Some(value) = some_value {
        println!("Value: {}", value);
    }
}
```

In the example above, we use `if let` to match the `Some` variant of the `Option` enum and bind the value to `value`. If the value is `Some`, the code block will be executed and the value will be printed. If the value is `None`, the code block will not be executed.