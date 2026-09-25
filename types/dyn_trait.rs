struct Dog {
    name: String,
    age: i32,
}
struct Cat {
    name: String,
    age: i32,
}
struct HumanBeing {
    name: String,
    age: i32,
}

trait Animal {
    fn bark(&self);
}

impl Animal for Dog {
    fn bark(&self) {
        println!("woof woof");
    }
}

impl Animal for Cat {
    fn bark(&self) {
        println!("meo meo");
    }
}

// fn animal_bark(a: impl Animal) {
//     a.bark();
// }

fn animal_bark(a: &dyn Animal) {
    a.bark();
}

fn main() {
    let d = Dog {
        name: "Leo".to_string(),
        age: 2,
    };

    let c = Cat {
        name: "Bunny".to_string(),
        age: 12,
    };
    let u = HumanBeing {
        name: "Steve".to_string(),
        age: 23,
    };

    animal_bark(&d);
    animal_bark(&c);

    // animal_bark(d);
    // animal_bark(c);
    //Rust effectively creates separate concrete versions:
    // animal_bark(Dog)
    // animal_bark(Cat)
}
