fn main() {
    let arr = [1, 2, 3, 4, 5, 6];

    let result: Vec<&i32> = arr.iter().filter(|x| **x % 2 == 0).collect();

    //arr.iter() creates an iterator whose items are &i32.
    println!("{:?}", result);
}
