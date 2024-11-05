# Error Handling

Errors are a fact of life in software, so ``Rust`` has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.

Rust groups errors into two major categories: ``recoverable`` and ``unrecoverable`` errors.

1. ``Recoverable`` errors, such as a file not found, it's reasonable to report the problem to the user and retry the operation.

2. ``Unrecoverable`` errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages don't distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn't have exceptions. Instead, it has something more similar to the ``error`` type in Go, or the new ``Expected`` type in C++.

# Table of Contents

1. [Unrecoverable Errors with ``panic!``](./1_unrecoverable_errors_with_panic.md)
2. [Recoverable Errors with ``Result``](./2_recoverable_errors_with_result.md)
