fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let a = 2;
    let b = 2.2;
    let n = String::from("Steve");
    print_type(&a);
    print_type(&b);
    print_type(&n);
}
