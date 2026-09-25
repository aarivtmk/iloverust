// type UserId = u64;
// type ProductId = u64;

// fn get_user(id: UserId) {
//     println!("userid is {}", id);
// }
// fn main() {
//     let product_id: ProductId = 42;

//     get_user(product_id); //  allowed — both are u64
// }
//
//
//
struct UserId(u64);
#[derive(Debug)]
struct ProductId(u64);

fn get_user(id: ProductId) {
    println!("userid is {:?}", id);
}
fn main() {
    let product_id: ProductId = ProductId(42);

    get_user(product_id);
}
/*



```rust
struct UserId(u64);
```

Read it as:

> Create a new type called `UserId` that contains one field of type `u64`.

So:

```text
UserId          → new type
   │
   └── field 0  → u64
```

When you write:

```rust
let id = UserId(42);
```

then:

```text
UserId
  └── 42        ← the actual u64 value
```

Compare with a normal named-field struct:

```rust
struct User {
    id: u64,
}
```

Here:

```text
User             → type
 └── id          → field name
      └── u64    → field type
```

But in:

```rust
struct UserId(u64);
```

there is **no field name**. The field is identified by its position:

```rust
id.0
```
 */
