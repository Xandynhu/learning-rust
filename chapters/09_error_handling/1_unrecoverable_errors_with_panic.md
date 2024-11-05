# Unrecoverable Errors with ``panic``!

Sometimes bad things happen in our code, and there is nothing we can do about it. For these cases, Rust has the ``panic!`` macro. There are 2 ways to cause a panic in practice:

1. We try to access an invalid memory location.
2. We call the ``panic!`` macro.

Let's try to calling the ``panic!`` macro:

```rust
fn main() {
    panic!("Crash and burn!");
}
```

When we run this code, we will see the following output:

```plaintext
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Let's now try to access an invalid memory location:

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

When we run this code, we will see the following output:

```plaintext
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


# Unwinding the Stack or Aborting in Case of a Panic

When a panic occurs, the program will start to unwind the stack, which means that Rust will clean up the memory that the program is using. If we don't want this behavior, we can use the ``panic=abort`` flag in the ``Cargo.toml`` file under the ``profile.release`` section. This will cause the program to immediately abort when a panic occurs, without unwinding the stack.

```toml
[profile.release]
panic = 'abort'
```