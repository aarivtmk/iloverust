trait Shape {
    const SIDES: u32;
}

struct Triangle;

impl Shape for Triangle {
    const SIDES: u32 = 3;
}

struct Square;

impl Shape for Square {
    const SIDES: u32 = 4;
}

fn main() {
    let t = Traingle;

    println!("{}", t.SIDES); // 3

    println!("{}", Square::SIDES); // 4
}
