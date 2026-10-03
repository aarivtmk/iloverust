// closures - anonymous function

fn main() {
    let e = 1000;
    // fn sub(a: i32, b: i32) -> i32 {
    //     a - d
    // }
    let sub = |a, b| a + b - e;

    let result = sub(1, 5);
    println!("result is {}", result);
}
