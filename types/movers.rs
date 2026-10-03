fn main() {
    let name = String::from("medha");
    let value = name.clone();
    let hello = move || {
        println!("name is {}", value);
    };
    println!("outisde name is {}", name);

    hello();
}
