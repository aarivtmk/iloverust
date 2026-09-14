pub struct student {
    name: Box<String>,
    age: i32,
}

fn main() {
    let s = student {
        name: Box::new("aariv".to_string()),
        age: 32,
    };
    let n: Box<f64> = Box::new(10.4);
    println!("n is {}", n);
    println!("student anme is {}", s.name);
}
