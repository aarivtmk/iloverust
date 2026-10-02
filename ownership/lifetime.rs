// fn read(x: &u64) {
//     println!("{}", x);
// }
// fn main() {
//     let x = 10;

//     {
//         let r = &x;
//         println!("r value inside block {}", r);
//     }
//     println!("r value outside block {}", r);
// }

// fn main() {
//     let r;
//     let x = 10;
//     // let r or let *r
//     // print *r / r both are same
//     // {}
//     {
//         let c;
//         r = &x;
//         c = 100;
//     }

//     println!("{}", *r);
//     println!("{}", c);
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

//

// <'a> means we are declaring a lifetime parameter named 'a for this function.
// fn longest_string(x: &str, y: &str) -> &str {
fn longest_string<'m>(x: &'m str, y: &'m str) -> &'m str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// a -----------
// b -----
fn main() {
    let result;
    let a = String::from("siliconvalleyusa");
    {
        let b = String::from("california");
        result = longest_string(&a, &b);
        println!("{}", result);
    }
    //     let a = String::from("mountainkid");
    //     let result;
    //     let b = String::from("nanda");
    //     result = longest_string(&a, &b);
    //     println!("{}", result);
}
