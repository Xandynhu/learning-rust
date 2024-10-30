# Defining an Enum

``Enums`` in ``Rust`` are similar to what we find in other languages like ``C`` or ``C++``. They are a way to define a type by enumerating its possible values.

For example, if we want to define a type that represents an ``IP`` address version, we can do it using an ``enum``:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

In this example, we defined an ``enum`` called ``IpAddrKind`` that has two possible values: ``V4`` and ``V6``. We can then use this ``enum`` to create instances of it:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Enums can also have data associated with each value. For example, if we want to represent an ``IP`` address, we can define an ``enum`` like this:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

In this example, we defined an ``enum`` called ``IpAddr`` that has two possible values: ``V4`` and ``V6``. Each value has a ``String`` associated with it. We can then use this ``enum`` to create instances of it:

```rust
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

There is also another advantage of using ``enums``. For example, we know that an ``V4`` IP address will always have 4 octets. If we wanted to store an ``V4`` address as 4 ``u8`` values while keeping the ``V6`` address as a ``String``, we can do it like this:

``NOTE:`` This is something that would be difficult to do with a ``struct``.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```


# The ``Option`` Enum and Its Advantages Over ``Null Values``

In ``Rust``, the ``Option`` ``enum`` is used to represent a value that can be ``Some`` value or ``None``. This is a very powerful feature of ``Rust`` that helps us avoid the `null` value problem that we find in other languages like ``C`` or ``C++``.

For example, if we want to request the first item in a non-empty list, we are going to have a value. But if we want to request the first item in an empty list, we would get nothing. Being able to represent this concept in the type system is very powerful because it allows the compiler to check if we are handling all the possible cases.

For all this, in ``Rust`` we don't have ``null`` values. Instead, we use the ``Option`` ``enum`` to represent the absence of a value.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The ``<T>`` syntax is going to be explained later in [chapter 10](../../10_generic_types_traits_and_lifetimes/readme.md). For now, we can think of it as a ``generic`` type that can be replaced by any other type.

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```
