// fn read(x: &u64) {
//     println!("{}", x);
// }
// fn main() {
//     let x = 10;

//     {
//         let r = &x;
//         println!("{}", r);
//     }
// }

// fn main() {
//     let r;

//     {
//         let x = 10;
//         r = &x;
//     }

//     println!("{}", r);
// }

// A lifetime is essentially:

// The region of execution during which a referenced value is guaranteed to remain valid for that reference.

// <'a> means we are declaring a lifetime parameter named 'a for this function.
// fn first(x: &str, y: &str) -> &str {
fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // let result;

    // let a = String::from("aariv");
    // let b = String::from("mountainkid");

    // result = first(&a, &b);

    // println!("{}", result);

    let a = String::from("aariv");
    let result;
    {
        let b = String::from("mountainkid");
        result = first(&a, &b);
        println!("{}", result);
    }
}
