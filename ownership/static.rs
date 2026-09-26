// static lifetime
fn main() {
    let x: &'static str = "hello";
    println!("{}", x);
    let r = greet(&x);
    println!("r is {}", r);
}

fn greet(message: &'static str) -> &'static str {
    println!("inside greet {}", message);
    message
}
