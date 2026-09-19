fn main() {
    // let arr: [i32; 23] = [2, 5, 6];

    // for value in arr {
    //     println!("arr element is {}", value);
    // }
    //
    //

    println!("x is {:?}", double(2));
}

fn double(x: i32) -> i32 {
    x * 2;
}

/*
*
* error[E0308]: mismatched types
  --> demo.rs:13:22
   |
13 | fn double(x: i32) -> i32 {
   |    ------            ^^^ expected `i32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
14 |     x * 2;
   |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
(base) Aariv@Aarivs-MacBook-Pro ~/projects/opensource/iloverust
*
 */
