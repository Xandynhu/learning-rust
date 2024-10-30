# The Slice Type

`Slices` let you reference a contiguous sequence of elements in a collection rather than the whole collection. It is a kind of ``reference``, so it does not have `ownership`.

## String Slices

A `string slice` is a reference to part of a ``String``. It looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

In this case, ``hello`` is a ``string slice`` that contains a reference to the first five bytes of ``s``. `world` is a ``string slice`` that contains a reference to the last five bytes of ``s``.

<p align="center">
    <img src="./assets/1_slice_string_reference.png" alt="String Slices" width="400px">
</p>

This is also a ``string slice``:

```rust
let s = String::from("hello world");

let hello = &s[..5];
let world = &s[6..];
```

## Other Slices

``Slices`` can be used with any type of collection:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```
