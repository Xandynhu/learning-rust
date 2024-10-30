# Packages and Crates

## Table of contents

1. [Crates](#crates)
2. [Packages](#packages)
3. [Idiomatic code Structure](#idiomatic-code-structure)

## Crates

A ``crate`` is the smallest amount of code that the Rust compiler considers at a time. ``Crates`` can contain modules, and modules can be defined in other files that get compiled with the crate.

A ``crate`` can come into two forms: a binary or a library.
- A ``binary crate`` is a program we can compile and execute.
- A ``library crate`` is a collection of functionality that we can use in other programs. Libraries do not contain a ``main`` function.

The ``crate root`` is a source file that the Rust compiler starts from and makes up the root module of your crate. A crate will have at most one library crate, but can have as many binary crates as you'd like.

## Packages

A ``package`` is a bundle of one or more crates that provides a set of functionality. A package contains a ``Cargo.toml`` file that describes how to build those crates.

After we run ``cargo new``, we have a package with a ``Cargo.toml`` file and a ``src`` directory. The ``src`` directory contains a ``main.rs`` file, which is the crate root of a binary crate with the same name as the package. The ``Cargo.toml`` file describes the package and contains metadata about the package.

In the example above we have a package that only contains ``src/main.rs``, meaning it only contains a binary crate. If we had a ``src/lib.rs`` file as well, the package would contain a library crate and a binary crate, both with the same name as the package.

## Idiomatic code Structure

When a project grows, it is common to have multiple programs that built up the project - like a server and a cli tool. In this case, both programs should be in the same package, under the ``src/bin`` directory.

Also, if there are helper pieces of code shared between the binary crates, it is common to extract that code into a library crate. This way, the binary crates can share the library crate.

For example, the project source structure should look like this:

For single files:

```plaintext
project
├── Cargo.toml
└── src
    ├── bin
    │   ├── server.rs
    │   └── client.rs
    └── lib.rs
```

For multiple files:

```plaintext
project
├── Cargo.toml
└── src
    ├── bin
    │   ├── server
    │   │   ├── main.rs       # Main entry point for the server binary
    │   │   └── handlers.rs   # Additional module for server-specific functionality
    │   └── client
    │       ├── main.rs       # Main entry point for the client binary
    │       └── utils.rs      # Additional module for client-specific utilities
    ├── lib.rs                # Library crate root, shared between server and client
    └── helpers
        ├── mod.rs            # Module declaration for helper functions in the library
        ├── config.rs         # Configuration handling code
        └── logging.rs        # Logging functionality shared by both binaries
```