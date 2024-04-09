# Mixed-C-Rust-C-project
An example of having a project with mixed C and Rust code

The C-part of the project is structured like so:
- Types: C header with base-types
- Math/add.c/h: addition implemented as a function
- Math/mult.h: extern function declaration to the Rust function (depends on add C function)
- Math/pow.c/h: power operation (depends on mult Rust function)

The Rust-part of the project is structured like so:
-  src/bindings.rs is generated with bindgen at compile time: contains all the extern functions and basetype defined in the C-parts.
- src/mult.rs: defines the mult function used in the C parts.

The tests can be found under the tests/ folder.