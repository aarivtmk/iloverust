fn main() {
    let mut x = 10;
    println!("address of x is {:p}", &x); // 0x16f096904

    // in c, let *a = &x;
    let a = &mut x; //
    let b = &a;
    println!("value of a is {}", a); // 10
    println!("address of a is {:p}", &a); // 0x16b072948
    println!("deref a is {}", *a);
    println!("{:p}", b);
    println!("{}", *b);
    println!("{}", **b);
}

// ownership
// let a = x;
// println!(a)
// println!(x)
// free,malloc, calloc -> garbage collection
fn main() {
    let a = String::from("nanda");
    // let a = 10; // i,u,f,
    println!("a is {}", a);
    let b = a;
    println!("a is {}", a);
    println!("a is {}", b);
}
// for simple traits , rust implements Copy trait
