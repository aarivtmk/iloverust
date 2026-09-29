fn main() {
    let name = String::from("aariv");
    let hello = || {
        println!("name is {}", name);
    };
    println!("outisde name is {}", name);

    hello();
}
