use std::mem::{align_of, size_of};

struct Cat {
    lives: u8,
    color: u32,
}

fn main() {
    println!("size = {}", size_of::<Cat>()); // 8
    println!("align = {}", align_of::<Cat>()); // 4
}
