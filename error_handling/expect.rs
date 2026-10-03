fn main() {
    let port = std::env::var("PORT").expect("PORT environment variable must exist");
    println!("port is {port}");
}
