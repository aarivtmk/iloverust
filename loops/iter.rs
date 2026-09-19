fn main() {
    let arr = [10, 20, 30];

    let mut iter = arr.iter();

    // match iter.next() {
    //     Some(value) => println!("i od have  a value"),
    //     None => println!("no value found"),
    // }

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
