use animal_mod::animal_trait::Animal;

pub struct Dog {
    pub age: i32,
    pub name: String,
}
impl Animal for Dog {
    fn bark(&self) {
        println!("woof woof");
    }
}
