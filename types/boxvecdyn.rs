trait Service {
    fn run(&self);
}

struct Hotel;
struct Taxi;

impl Service for Hotel {
    fn run(&self) {
        println!("Hotel");
    }
}

impl Service for Taxi {
    fn run(&self) {
        println!("Taxi");
    }
}

fn main() {
    let numbers = vec![10, 20, 30];
    let x = Box::new(10);
    // Stack          Heap
    // x ─────────→  10
    //
    let values: Vec<Box<i32>> = vec![Box::new(10), Box::new(20), Box::new(30)];
    println!("values are {:?}", values);
    let services: Vec<Box<dyn Service>> = vec![Box::new(Hotel), Box::new(Taxi)];
    services[1].run();
}

/*
Static dispatch
    ↓
Compiler decides
    ↓
Fast/direct call

Dynamic dispatch
    ↓
Runtime decides
    ↓
vtable → correct method
 */
