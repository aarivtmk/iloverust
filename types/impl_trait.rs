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

trait Human {
    fn speak(&self);
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

impl Human for HumanBeing {
    fn speak(&self) {
        println!("Hey Hey");
    }
}

// fn animal_bark<T: Animal>(a: T) {
//     a.bark();
// }

fn animal_bark(a: impl Animal) {
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

    animal_bark(d);
}
