Cobbling an Interpreter, June 24-28, 2024
=========================================

Goal: Implement an interpreter in Rust from the first part of
[Crafting Interpreters](https://craftinginterpreters.com)
[Rust Programming Language](https://www.rust-lang.org)

A few tutorials:

* [Comprehensive Rust](https://google.github.io/comprehensive-rust/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)


Topics to Focus On
------------------

The focus of the week is on the somewhat narrow problem of
implementing the Lox interpreter from Crafting Interpreters.  As such,
it's definitely NOT necessary to know all of Rust.  However, the
following topics might serve as a guide when
reviewing some Rust basics:

* Text and string manipulation.  We'll need to write a scanner and parser.

* Recursion.  Maybe of the algorithms and data structures used in
  implementing an interpreter involve recursion.

* Trees.  We'll need to represent an abstract syntax tree (AST)
  and perform various tasks while traversing the tree.

* Pattern matching.  We'll need to write a lot of code involving
  pattern matching over AST tree nodes.

* Environments. An interpreter typically involves a mutable runtime
  environment of variables (stack frames, etc.) that gets built
  using hash-maps or trees of some kind.