# Storing Keys with Associated Values in Hash Maps

Here we'll go over the basic API of hash maps, how to create them, and how to update, read, and remove elements.

## Table of Contents

1. [Creating a New Hash Map](#creating-a-new-hash-map)
2. [Accessing Values in a Hash Map](#accessing-values-in-a-hash-map)
3. [Hash Maps and Ownership](#hash-maps-and-ownership)
4. [Updating a Hash Map](#updating-a-hash-map)

## Creating a New Hash Map

We can create a new hash map with the ``HashMap::new`` function and then add elements with the ``insert`` method.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

## Accessing Values in a Hash Map

We can access the value associated with a particular key with the ``get`` method.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

## Hash Maps and Ownership

For types that implement the ``Copy`` trait, like ``i32``, the values are copied into the hash map. For owned values like ``String``, the values will be moved and the hash map will be the owner of those values.

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// field_name and field_value are invalid at this point
```

## Updating a Hash Map

We can update a hash map by overwriting a value with the ``insert`` method, or by using the ``entry`` method to only insert if the key does not already have a value.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

scores.insert(String::from("Blue"), 25);

scores.entry(String::from("Yellow")).or_insert(50);
```
