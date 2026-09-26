fn main() {
    let mut x = 10;
    println!("address of x is {:p}", &x); // 0x16f096904
    let a = &mut x; //
    println!("value of a is {}", a); // 10
    println!("address of a is {:p}", &a); // 0x16b072948
    println!("deref a is {}", *a);
}

// ownership
// let a = x;
// println!(a)
// println!(x)


let a = String::from("nanda");
println!("a is {}",a);
let b = a;
println!("a is {}",a);
println!("a is {}",b);
