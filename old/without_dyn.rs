struct Dog;
struct Cat;
trait Animal {
    fn speak(&self);
}
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}
fn main() {
    let dog = Dog;
    Dog::speak(&dog);
}
