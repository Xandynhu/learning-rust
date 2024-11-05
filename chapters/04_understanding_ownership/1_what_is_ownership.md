# What is Ownership?

``Ownership`` a set of rules that govern how a ``Rust`` program manages ``memory``. These rules are checked by the Rust compiler at compile time.

# Table of Contents

1. [The Stack and the Heap](#the-stack-and-the-heap)
2. [Ownership Rules](#ownership-rules)
3. [Variable Scope](#variable-scope)
4. [The ``String`` Type](#the-string-type)
5. [Variables and Data Interaction: Move vs Clone](#variables-and-data-interaction-move)
    1. [Why ``heap`` data is moved instead of copied](#why-heap-data-is-moved-instead-of-copied)
    2. [How ``Rust`` solves the double ownership problem](#how-rust-solves-the-double-ownership-problem)
6. [Ownership and Functions](#ownership-and-functions)
7. [Return Values and Scope](#return-values-and-scope)


## The Stack and the Heap

The ``stack`` and the ``heap`` are, essentially, just two different parts of the memory used by a program to store data.

- The ``stack`` is used for data that has a known, fixed size at compile time. Faster.

- The ``heap`` is used for data that has an unknown size at compile time or a size that might change. Slower.


## Ownership Rules

1. Each ``value`` must always have an ``owner``.
2. There can only be one ``owner`` at a time for a given ``value``.
3. When the ``owner`` goes out of ``scope``, the ``value`` will be ``dropped``.


## Variable Scope

The ``scope`` is the range within a program for which a variable is valid.

```rust
                     // 's' is not valid here, it’s not yet declared
if something {
    let s = "hello"; // 's' isvalid from this point forward

    do_something(s); // do stuff with 's'
}
                     // this scope is now over, and 's' is no longer valid
```

## The ``String`` Type

The ``String`` type is different from `string literals` we have seen before.

- ``string`` literals are immutable and hardcoded into the program. Goes into the ``stack``.
- ``String`` type is can be declared as mutable and at runtime. Goes into the ``heap``.

```rust
let s1 = "hello";               // string literal
let s2 = String::from("hello"); // String type
```

The `::` operator allows us to namespace this particular ``from`` function under ``String`` rather than using some sort of name like ``string_from``. Similar to what we have in other languages like ``C++``.

This kind of ``String`` can be mutated.

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s);      // This will print `hello, world!`
```


## Variables and Data Interaction: Move

Multiple variables can interact with the same data in different ways in ``Rust``.

```rust
// Because 'x' and 'y' are both stored on the stack, the
// following code is valid and works as a simple copy
let x = 5;      // 'x' is bound to 5
let y = x;      // 'y' is bound to 5 through 'x'

println!("x = {}, y = {}", x, y);
```

However, when we move a ``String`` type, the following code will not work.

```rust
// Because s1 is stored on the heap, the following code
// will work as a move, not a copy, when we assign the
// value of s1 to s2
let s1 = String::from("hello"); // 's1' is bound to "hello"
let s2 = s1;                    // s2 is bound to "hello" through 's1'
                                // 's1' is no longer valid: moved to s2

println!("{}, world!", s1);     // This will not work: 's1' is no longer valid
```

Both pieces of code look similar but, in the first example, both `x` and `y` are instances of a primitive type - which are handled by the ``stack``. In the second example, `s1` and `s2` are instances of the ``String`` type - which are handled by the ``heap``.

Assigning values stored in the ``stack`` are handled as a `copy`. Assigning values stored in the ``heap`` are handled as a ``move``.

When a variable moves, the `ownership` of the data is transferred to the new variable. The old variable is no longer valid.

### Why ``heap`` data is ``moved`` instead of ``copied``:

Let's take the ``String`` type, again, as an example. We know that it can be expanded at runtime. Therefore, it is stored on the ``heap``. Under the hood, these types of data are stored as a ``pointer`` (in the ``stack``) to the actual data (in the ``heap``).

<p align="center">
    <img src="./assets/1_string_memory_allocation.png" alt="String Memory Allocation" width="400">
</p>

Now, let's create another variable and assign the value of the first ``String`` variable to it.

``NOTE:`` we don't want to do unecessary copies of data in the ``heap`` because it can be expensive. So we copy only the ``pointer`` that points to the data.

<p align="center">
    <img src="./assets/2_double_ownership.png" alt="String Memory Allocation: double ownership" width="400">
</p>

Now, what we have here is effectivelly 2 different variables pointing to the same data in the ``heap``. So now both variables ``own`` that data. This is a problem because we are violating the [seccond ownership rule](#ownership-rules) that says that there can only be one ``owner at a time`` for a given ``value``.


### How ``Rust`` solves the double ownership problem?

There are 2 ways to solve this problem:
1. `Transfering Ownership`: The first variable that had the data is no longer valid. This is called a ``move``.

<p align="center">
    <img src="./assets/3_solution_move.png" alt="String Memory Allocation: move" width="400">
</p>

```rust
let s1 = String::from("hello"); // 's1' is bound to "hello"
let s2 = s1;                    // s2 is bound to "hello" through 's1'
                                // 's1' is no longer valid: moved to s2

println!("{}, world!", s2);
```

2. ``Cloning the Data``: The data is copied to the new variable. This is called a ``clone``.

<p align="center">
    <img src="./assets/4_solution_clone.png" alt="String Memory Allocation: clone" width="400">
</p>

```rust
let s1 = String::from("hello"); // 's1' is bound to "hello"
let s2 = s1.clone();            // 's2' is bound to "hello" through a clone of 's1'

println!("{}, world!", s1);
println!("{}, world!", s2);
```



## Ownership and Functions

When passing a variable to a function, the ownership principals are still valid and the idea is practically the same as when assigning a variable to another.

In the case of ``functions``, the variable can be ``moved`` or ``copied`` to the function as an argument.

```rust
// String type as an argument:
let s = String::from("hello"); // 's' is bound to "hello"
do_something(s);               // 's' is moved to the function and is no longer valid
                               // this is because the function's argument takes
                               // ownership of the variable

// Primitive type as an argument:
let x = 5;                     // 'x' is bound to 5

do_something(x);               // 'x' is copied to the function
                               // this is because the function's argument takes
                               // a copy of the variable

println!("x: {}", x);          // This will work
```

More details on the [variables and data interaction](#variables-and-data-interaction-move) section.

## Return Values and Scope

When a function returns a value, the ownership of that value is also transferred to the calling function.

```rust
fn create_string() -> String {
    let s = String::from("hello"); // 's' is bound to "hello"
    s                              // 's' is returned and moved to the calling function
}

let s = create_string();          // 's' is bound to "hello"
                                  // the value "hello" is moved from the funciton return
                                  // variable to the calling function variable 's'
```

More details on the [variables and data interaction](#variables-and-data-interaction-move) section.