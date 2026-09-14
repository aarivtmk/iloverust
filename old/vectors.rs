struct Dog;
struct Cat;

trait Animal {
    fn speak(&self);
}

fn main() {
    let num = vec![1, 2, 5, 6];
    println!("vec is {:?}", num[1]);

    let names = vec![Box::new(String::from("a")), Box::new(String::from("b"))];
    println!("names are {:?}", names);
}
