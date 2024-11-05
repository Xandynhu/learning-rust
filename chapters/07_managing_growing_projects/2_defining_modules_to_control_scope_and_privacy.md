# Defining Modules to Control Scope and Privacy

In this section we are going to see ``modules`` and other parts of the module system called ``paths``, which allow us to name items and control their visibility and privacy. The ``use`` keyword, which is used to bring a path into scope. The ``pub`` keyword to make items public - so we can ``use`` them in other parts of our code. And the ``as`` keyword to rename items when we bring them into scope.

# Table of Contents

1. [Modules Cheat Sheet](#modules-cheat-sheet)
2. [Grouping Related Code in Modules](#grouping-related-code-in-modules)

## Modules Cheat Sheet

Rust compiler "point of view" when compiling a project:

1. **Start from the crate root:** When compiling a crate, the compiler starts from the crate root (usually ``src/main.rs`` or ``src/lib.rs``) and follows the module tree based on the ``mod`` and ``use`` keywords.
2. **Decalring modules:** In the crate root file, we can declare new modules: lets say we declare a module called ``garden`` with ``mod garden``. Now, the compiler will look for the module's code in these places:
    1. **inline:** within curly brackets that replace that replace the semicolon following ``mod garden``.
    2. **external file:** in a file named ``garden.rs``
    3. **external directory:** in a file inside another directory, like ``garden/mod.rs``.
3. **Declaring submodules:** Inside a module, we can declare submodules using the same rules as above. Lets say we declare ``vegetables`` module inside the ``garden`` module. The compiler will look for the module's code in these places:
    1. **inline:** within curly brackets that replace that replace the semicolon following ``mod vegetables``.
    2. **external file:** in a file named ``vegetables.rs``
    3. **external directory:** in a file inside another directory, like ``vegetables/mod.rs``.
4. **Private vs Public:** By default, items in a module are private. We can make them public using the ``pub`` keyword. This allows us to use them in other parts of our code.
5. **The ``use`` keyword:** We can bring a path into scope using the ``use`` keyword. This allows us to use the items in the path without having to write the full path every time.

Let's imagine a binary crate named ``backyard`` that illustrates these concepts:

abc`a`a`cbaplaintext
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case is ``src/main.rs``, and it contains:

abc`a`a`cbarust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

## Grouping Related Code in Modules

``Modules`` let us organize our code within a crate. We can group related code together, and control the visibility and privacy of items. This makes our code easier to read and maintain.

As an example, let's write a library crate that provides the functionality of a restaurant. We'll start by creating a new library crate:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

We define a module with the ``mod`` keyword, followed by the module's name. We can nest modules inside other modules. In this case, we have a module called ``front_of_house`` that contains two submodules: ``hosting`` and ``serving``.

Earlier, we mentioned that the ``src/main.rs`` and ``src/lib.rs`` files are called the ``crate root`` files. The module tree starts from these files. In this case, the module tree starts from the crate root file, and the ``front_of_house`` module is a child of the crate root.

```plaintext
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

This tree shows some of the modules nested inside other modules. The tree also shows that some of the modules are ``siblings`` - they are at the same level in the module tree, like ``hosting`` and ``serving``.