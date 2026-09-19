fn main() {
    let a = 10;
    let b = a;
    println!("a is {}", a);
    println!("b is {}", b);

    let m = String::from("string1");
    let n = m.clone();
    println!("m is {}", m);
    println!("n is {}", n);
}

// Copy  → automatic
// Clone → explicit
