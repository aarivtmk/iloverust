## Problem Set
1. Explain `let x: u64 = 10

```text
STACK

┌──────────────┐
│ x = 10       │
├──────────────┤
│ ...          │
└──────────────┘

```
2.```let x = String::from("hello");```
```text
STACK                         HEAP

x
┌───────────────┐             ┌─────────────┐
│ pointer ────────────────→   │ h e l l o   │
│ length        │             └─────────────┘
│ capacity      │
└───────────────┘
```

3. answer the following questions
```rust
fn main() {
    let x: u64 = 10;
    let y = x;
    println!("{}", y);
}
```
- What is x?
- What is y?
- Are there one or two 10 values?
- Where are they stored conceptually?
- Why does Rust allow y = x without making x unusable?

4. OUTPUT
```rust 
let mut x = 10;
x = 20;
```

5. is the binding mutable?
```let mut x = 10;```

6. Output of this code

```rust
let mut x = String::from("hello");
x.push_str(" world");
```
7. Output of this code
```rust
    let x = if true {
        10
    } else {
        20
    };
```

8. Expressions produce value
- True or False?

9. Explain
```rust
let x = {
    let a = 10;
    let b = 20;

    a + b
};
```
10. Blocks are expressions , true or false?

11. output of this code
```rust
let x = {
    10
};
println!("x is {:?}", x);
```

12. output of this code
```rust
let x = {
    10;
};
println!("x is {:?}", x);
```

13. Statements return ()/unitvalue - there is no meaningful value to return

14. explain
```rust
let result = if price > threshold {
    buy()
} else {
    wait()
};
```

15. Parameters vs arguments
```rust 
fn add(a: i32, b: i32) -> i32 
```
16. output of this code
```rust
fn add(a: i32, b: i32) -> i32 {
    "hello"
}
```


17. draw the memory layout for the following code

```rust
fn main() {
    let result = add(10, 20);
}
```

```text
STACK

┌────────────────────┐
│ add() frame        │
│ a = 10             │
│ b = 20             │
│ temporary values   │
├────────────────────┤
│ main() frame       │
│ result ...         │
└────────────────────┘

```
18. difference
```rust
let x: u8 = 10;
let y: u64 = 10;
```

19. On phunsuk we have 1 million users, and whats the best type for age variable to store?

20.
```rust
integers
floating point
boolean
characters
tuples
arrays
structs
enums
references
pointers
functions
traits
generic types
```

21. explain the following
```rust
let x: u8 = 0b00001101;
```
22. explain the following
```rust
let mask = 0x0F;
```
23. if is a decision, true or not?
```rust
if price > 100 {
    buy();
} else {
    wait();
}
```
24. whats this? `&& , ||, !`

25. explain Display trait
26. explain padding 
27. explain alignment - u8, alignment ___, u64 alignment ____
28.write rust code for mutable reference
