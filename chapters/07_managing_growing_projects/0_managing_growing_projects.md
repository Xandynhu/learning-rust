# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

As we write large programs, organizing our code will become increasingly important. By grouping related functinality and separating code with distinct features, we can make our code easier to understand and maintain. In ``Rust``, we can use ``packages``, ``crates``, and ``modules`` to organize our code.

1. The programs we've written so far in this repository have all **been in one module in one file**.

2. As a project grows, we should organize code by splitting it into `multiple modules` and then `multiple files`. This chapter will cover how to do that.

3. A ``package`` can contain **multiple** ``binary crates`` and **optionally one** ``library crate``.

4. As a package grows, we can extract parts into separate ``crates`` that other packages can use. They will become ``external dependencies``.

This chapter will cover all these techniques and a bit more.

For very large projects, we can also use ``workspaces`` to manage multiple packages that are related to each other. We can have more details on ``workspaces` in the [chapter 14](../14_more_about_cargo_and_crates_io/readme.md).

Rust has a number of features that allow us to manage and organize our code, selecting whether to make our code ``public`` or ``private``, and what ``names`` to ``expose`` in each ``scope``. These features can be referred to as the ``module system``.

1. **``Packages:``** A ``Cargo`` feature that lets us build, test, and share ``crates``.
2. **``Crates:``** A ``tree of modules`` that produces a library or executable.
3. **``Modules`` and ``use``:`** Lets us control the organization, scope, and privacy of paths.
4. **``Paths:``** A way of ``naming`` an item, such as a struct, function, or module.


## Table of Contents

1. [Packages and Crates](./1_packages_and_crates/readme.md)
2. [Defining Modules to Control Scope and Privacy](./2_defining_modules_to_control_scope_and_privacy/readme.md)
3. [Paths for Referring to an Item in the Module Tree](./3_paths_for_referring_to_an_item_in_the_module_tree/readme.md)
4. [Bringing Paths into Scope with the `use` Keyword](./4_bringing_paths_into_scope_with_the_use_keyword/readme.md)
5. [Separating Modules into Different Files](./5_separating_modules_into_different_files/readme.md)
