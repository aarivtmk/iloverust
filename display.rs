fn print_num<T: std::fmt::Display>(num: T) {
    println!("num is {}", num);
}

fn main() {
    print_num(2);
}
