# Defining and Instantiating Structs

In `Rust`, we can define a `struct` using the `struct` keyword. A `struct` is a custom data type that lets you name and package together multiple related values that make up a meaningful group.

Here is an example of a `struct` definition:

```rust
struct User {
    active:        bool,
    username:      String,
    email:         String,
    sign_in_count: u64,
}
```

We can create and use an instance of the `User` `struct` like this:

```rust
fn main() {
    // Create an instance of the User struct
    let mut user1 = User {
        active:        true,
        username:      String::from("someone"),
        email:         String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Access the fields of the User struct
    println!("Username: {}", user1.username);

    // Change username field of the User struct
    user1.username = String::from("someone_else");
    println!("Username: {}", user1.username);
}
```

The `struct` definition is a blueprint for the `struct`, and instances of the `struct` are the actual objects that we can create based on the `struct` definition.

`NOTE`: In order to change the value of a field in a `struct`, the entire instance must be mutable. We can make an instance mutable by using the `mut` keyword after the `let` keyword.


