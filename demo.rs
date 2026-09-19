// fn main() {
//     // let arr: [i32; 23] = [2, 5, 6];

//     // for value in arr {
//     //     println!("arr element is {}", value);
//     // }
//     //
//     //

//     println!("x is {:?}", double(2));
// }

// fn double(x: i32) -> i32 {
//     x * 2;
// }

// fn print_num<T: std::fmt::Display>(n: T) {
//     println!("n is {}", n);
// }

// fn main() {
//     print_num(2);
//     print_num("Nanda");
// }

// struct User - username, age
// trait  - UserDetails
// in trait - we have find_username function

// implement a trait bound funciton - fetch user_name on T:UserDetails

fn main() {
    let mut v = Vec::new();
    println!("vector before for loop {:?}", v);
    for i in 0..10 {
        v.push(i);
    }
    println!("vector after for loop {:?}", v);
}
