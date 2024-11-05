# Storing UTF-8 Encoded Text with Strings

We already saw a bit about ``Strings`` back in [chapter 4](./../04_understanding_ownership/1_what_is_ownership.md).

But apparently new Rustaceans commonly get stuck on strings for a combination of 3 reasons:

1. Rust's propensity for exposing errors at compile time.
2. Strings being a more complicated data structure than most devs give them credit for.
3. UTF-8 encoding.

So let's dive a bit deeper into the topic of ``Strings``.

# Table of Contents

1. [What is a String?](#what-is-a-string)
2. [Creating a New String](#creating-a-new-string)
3. [Updating a String](#updating-a-string)
4. [Concatenation with Strings](#concatenation-with-strings)

## What is a String?

We'll first define a ``String``, like we saw in [chapter 4](./../04_understanding_ownership/1_what_is_ownership.md), as a growable, mutable, owned, UTF-8 encoded string type. It is fundamentally different from the string slice type, ``&str``.

## Creating a New String

Many of the same operations available for ``Vec<T>`` are available for ``String``. This is because ``String`` is a wrapper around a ``Vec<u8>``, but with some extra guarantees, restrictions, and capabilities.

```rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();

let s = "initial contents".to_string();
```

## Updating a String

Now we know that ``Strings`` can grow in size, but how do we do that?

Appending to a String with `push_str` and `push`:

```rust
let mut s = String::from("foo");
s.push_str("bar");

let mut s = String::from("foo");
s.push('l');
```

## Concatenation with Strings

```rust
// Using the `+` operator
let s1 = String::from("foo");
let s2 = String::from("bar");
let s3 = s1 + &s2;
let s4 = s1 + "-" + &s2;

// Using the `format!` macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```
