# Paths to Referring to an Item in the Module Tree

To show Rust where to find an item in the module tree, we use a path similar to a filesystem path. We can use two types of paths:

1. **The ``absolute`` path:** This starts from the crate root; for code from an external crate, the absolute path starts from the crate root of the external crate; and for code in the current crate, it starts with the literal ``crate`` keyword.
2. **The ``relative`` path:** This starts from the current module and uses ``self``, ``super``, or an identifier in the current module.

Both ``absolute`` and ``relative`` paths are followed by one or more identifiers separated by double colons (``::``).

Using something similar to our previous example, for a restaurant we would get something like this:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Exposing Paths with the ``pub`` Keyword

When we bring a module into scope with the ``use`` keyword, the path to the module is exposed. If we want to bring a module into scope and make the path private, we can use the ``as`` keyword to bind the path to a different name.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist as add;

fn main() {
    hosting::add_to_waitlist();
    add();
}
```

## Starting Relative Paths with ``super``

In summary, we can use the ``super`` keyword to bring into scope an item that we know is in the parent module.

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

In the example above, we can see that the ``cook_order`` function is in the ``back_of_house`` module, and the ``serve_order`` function is in the parent module. We can use the ``super`` keyword to bring the ``serve_order`` function into scope from the parent module.