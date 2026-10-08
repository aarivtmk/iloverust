#[derive(Debug)]
struct User {
    name: String,
}

// derive says - please automatically write some common implementation for this type.

// without derive, printing user wont work since its complex type

// so instead of implementing Debug, we are asking rust with derive macro to implement dDebug trait for us.

/*
*
* You write:

#[derive(Debug)]
struct User { ... }

             ↓

derive macro

             ↓

generates:

impl Debug for User { ... }

             ↓

Rust compiler compiles it
*/
