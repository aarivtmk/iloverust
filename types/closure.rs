fn main() {
    let d = 1000;
    // let sub = |a, b| a - d;
    fn sub (a,b)->i32{
        a-d
    }
    let result = sub(1, 5);
    println!("result is {}", result);
}
