fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let arr = [1, 2, 3, 4];
    // map takes the reference of the element and transforms the value
    let result: Vec<i32> = arr.iter().map(|x| *x * 2).collect();

    // alternative way

    let result2 = arr.map(|x| x * 2);
    println!("result is {:?}", result2);
}
