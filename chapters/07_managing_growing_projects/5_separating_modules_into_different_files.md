# Separating Modules into Different Files

So far we've been only talking about defining modules in one file. But as our Rust modules grow, we might want to move their definitions into a separate file to make the code easier to navigate.

Let's say we have a module named `sound` that we defined in the `lib.rs` file. We can move the `sound` module into its own file by creating a new file named `sound.rs` in the `src` directory. Then we can move the `sound` module definition into the `sound.rs` file.

Here's how we can do that:

```rust
// src/lib.rs
mod sound;
```

```rust
// src/sound.rs
pub mod instrument {
    pub fn clarinet() {
        // Function body code would go here
    }
}
```

Now we can use the `sound` module in the `lib.rs` file as we did before:

```rust
// src/lib.rs
mod sound;

use sound::instrument;

fn main() {
    instrument::clarinet();
}
```

This way, we can split our code into multiple files and keep it organized. We can also nest modules in different files by creating subdirectories in the `src` directory. For example, we can create a `sound` directory in the `src` directory and move the `sound` module into a file named `instrument.rs` in the `sound` directory. Then we can define a sub-module named `voice` in a file named `voice.rs` in the `sound` directory.
