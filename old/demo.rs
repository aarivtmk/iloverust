fn main() {
    let a: i32 = 2;
    {
        let a = String::from("medha");
        let b = 5;
        println!("value of b is {}", b);
        println!("value of a inside is {}", a);
    }
    println!("value of a outside is {}", a);
}
