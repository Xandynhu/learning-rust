# Bringing Paths into Scope with the ``use`` Keyword

This subchapter will cover how to bring paths into scope with the ``use`` keyword. We will also cover how to create idiomatic ``use`` paths, provide new names with the ``as`` keyword, re-export names with ``pub use``, use external packages, and use nested paths to clean up large use lists.

It is also meant to keep everything simple, easy to understand, with not much text and more code examples.

# Table of Contents

1. [Creating Idiomatic ``use`` Paths](#creating-idiomatic-use-paths)
2. [Providing New Names with the ``as`` Keyword](#providing-new-names-with-the-as-keyword)
3. [Re-exporting Names with ``pub`` ``use``](#re-exporting-names-with-pub-use)
4. [Using External Packages](#using-external-packages)
5. [Using Nested Paths to Clean Up Large use Lists](#using-nested-paths-to-clean-up-large-use-lists)

## Crating Idiomatic ``use`` Paths

In Rust, idiomatic ``use`` paths are intended to import only the necessary items, keeping imports concise and organized.

An idiomatic ``use`` path should bring **only the required elements** into scope, avoiding excessive or overly broad imports, which can make code cleaner and more maintainable.

```rust
// Importing only the necessary item from a module
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("{:?}", map);
}
```

This example imports only the ``HashMap`` type from the ``std::collections`` module.


## Providing New Names with the ``as`` Keyword

The ``as`` keyword allows us to bring items into scope with a new, potentially more convenient or descriptive name.

When importing multiple items with similar names or for clarity, ``as`` can rename imported items to avoid conflicts or improve readability.

```rust
use std::io::Result as IoResult;

fn main() -> IoResult<()> {
    println!("This is a renamed Result type from std::io");
    Ok(())
}
```

In this example, we renamed ``std::io::Result`` to ``IoResult`` to differentiate it from other ``Result`` types.

## Re-exporting Names with ``pub use``

With ``pub use``, we can re-export items from other modules, making them accessible to other modules that use this module.

Re-exporting items with ``pub use`` allows encapsulation of module paths while exposing the item to outer modules as if it were defined in the current module.

```rust
mod outer {
    pub mod inner {
        pub fn inner_function() {
            println!("Called inner function!");
        }
    }

    pub use inner::inner_function;
}

fn main() {
    outer::inner_function();
}
```

Here, ``inner_function`` is re-exported with ``pub use``, allowing it to be accessed directly through ``outer`` instead of ``outer::inner``.

## Using External Packages

External packages, also known as crates, are brought into scope in Rust using the ``use`` keyword after specifying them in ``Cargo.toml``.

To use external packages, first add them to your project’s dependencies in ``Cargo.toml``. Then, use ``use`` to import the necessary parts of the package.

```toml
[dependencies]
rand = "0.8.4"
```

```rust
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", random_number);
}
```

In this example, we import the ``Rng`` trait from the ``rand`` crate to generate a random number.

## Using Nested Paths to Clean Up Large ``use`` Lists

Nested paths help clean up ``use`` statements by allowing multiple imports from the same module to be combined into a single line.

Instead of importing multiple items with separate ``use`` statements, nested paths allow multiple imports from the same module or crate to be specified in a single statement, reducing redundancy.

```rust
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");

    let result = 3.cmp(&2);
    println!("{:?}, {:?}", map, result);
}
```

In this example, we use nested paths to import both ``Ordering`` from ``std::cmp`` and ``HashMap`` from ``std::collections`` in a single line.
