fn main() {
    let arr = [1, 2, 3, 4];
    // 3,6,9,12

    let result: Vec<i32> = arr.iter().map(|x| x * 3).filter(|x| x % 2 == 0).collect();

    //arr.iter() creates an iterator whose items are &i32.
    println!("{:?}", result);
}
