// Associated type = a type placeholder
// declared inside a trait, which each implementation chooses.
trait Container {
    type Item;
    fn get(&self) -> Self::Item;
}

struct Numbers;

impl Container for Numbers {
    type Item = u32;

    fn get(&self) -> u32 {
        42
    }
}

struct Names;

impl Container for Names {
    type Item = String;

    fn get(&self) -> String {
        String::from("Aariv")
    }
}

fn main() {
    let n = Numbers;
    println!("{:?}", n.get());
}
