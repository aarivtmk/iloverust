fn main() {
    let mut x = 10;
    println!("address of x is {:p}", &x);
    let a = &mut x;
    println!("value of a is {:p}", a);
    println!("address of a is {:p}", &a);
    println!("deref a is {}", *a);
}
