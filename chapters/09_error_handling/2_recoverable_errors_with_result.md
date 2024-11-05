# Recoverable Errors with ``Result``

Most errors in Rust aren't serious enough to cause the program to stop entirely. Sometimes when a function fails it's for a reason that we can easily interpret and respond to. For example, if we try to open a file and that file doesn't exist, we might want to create the file instead of terminating the process.

In Rust, we use the ``Result`` enum to handle recoverable errors. Let's recall how this enum is defined:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The ``Result`` enum has two variants: ``Ok`` and ``Err``.

- The ``Ok`` variant indicates that the operation was successful and contains a value of the success type.
- The ``Err`` variant indicates that an error occurred and contains a value of the error type.

Here's an example of a function that reads a file and returns the content of the file as a string:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

The return type of the ``File::open`` function is a ``Result<T, E>``. If the file exists, the function will return an ``Ok`` variant with a file handle. If the file doesn't exist, the function will return an ``Err`` variant with an error value of type ``std::io::Error``.

To continue our code, we need to address both cases. We can use a ``match`` expression to handle the ``Result``:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file)   => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
}
```

## Matching on Different Errors

The code above will ``panic`` no matter what the error is. However, if we want to take different actions depending on the error, we can match on the different error values:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file)   => file,
        Err(error) => match error.kind() {

            // If the file doesn't exist, create it
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file)   => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },

            // If there is another error, panic
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

## Shortcuts for Panic on Error: ``unwrap`` and ``expect``

Using ``match`` works well, but it can be verbose. Rust provides two methods, ``unwrap`` and ``expect``, that allow us to handle errors more concisely.

1. The ``unwrap`` method returns the value inside the ``Ok`` variant if the operation was successful. If the operation was not successful, the method will call the ``panic!`` macro and display the error message.

    From my personal perspective, ``unwrap`` goes against the Rust philosophy of handling errors and its idea of being a safe language. So I would avoid using it.

2. The ``expect`` method is similar to ``unwrap``, but it allows us to specify the error message that will be displayed if the operation fails.

    Again, I would avoid using ``expect`` for the same reasons as ``unwrap``. Even if it is more informative and I can see places where it would fit, I would still avoid it - not as a rule, but as a guideline.


## Propagating Errors

When a function`s implementation call something that might fail, it can either handle the error or propagate it to its caller, so it can handle the error. This is done by returning the error to the caller.

Here's an example of a function that reads a file and returns the content of the file as a string:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

## A Shortcut for Propagating Errors: the ``?`` Operator

The code above can be simplified using the ``?`` operator, which is used to propagate errors. If the value is an ``Ok`` variant, the value inside the ``Ok`` variant is returned. If the value is an ``Err`` variant, the ``Err`` value is returned from the function.

Here's the same function as above, but using the ``?`` operator:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

Again, this is something that I would avoid using. I would prefer to be explicit about the error handling and not rely on the ``?`` operator.

From my perspective, it makes the code ``smaller``, but not necessarily ``easier`` to read. Maybe it is just a matter of getting used to it...
