// math.rs
fn print_result() {
    println!("here is the result");
}
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: i32,
}
