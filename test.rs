struct User {
    name: String, // medha
    age: i32,     // 18
}

fn print_data<T: std::fmt::Display>(n: T) {
    println!("n is {}", n);
}

fn main() {
    // let u = User {
    //     name: "Steve".to_string(),
    //     age: 23,
    // };
    print_data("one");
    print_data(2.3);
    print_data(2);
}
