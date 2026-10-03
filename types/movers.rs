fn main() {
    let name = String::from("medha");
    let hello = move || {
        println!("name is {}", name);
    };
    println!("outisde name is {}", name);

    hello();
}
