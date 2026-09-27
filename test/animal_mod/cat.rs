use animal_mod::animal_trait::Animal;
pub struct Cat {
    pub name: String,
    pub age: i32,
}
impl Animal for Cat {
    fn bark(&self) {
        println!("meo meo");
    }
}
