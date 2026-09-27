mod animal_mod;
use animal_mod::animal_trait::Animal;
use animal_mod::cat::Cat;
use animal_mod::dog::Dog;
fn main() {
    let d = Dog {
        name: "Leo".to_string(),
        age: 2,
    };

    let c = Cat {
        name: "Bunny".to_string(),
        age: 12,
    };
    c.bark();
    d.bark();
}
