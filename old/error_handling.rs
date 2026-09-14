// Result<Ok,Err>

fn divide(n1: i32, n2: i32) -> Result<i32, String> {
    if n2 != 0 {
        Ok(n1 / n2) // Ok(2)
    } else {
        Err(String::from("can't divide by 0"))
    }
}

// fn cal(a: i32, b: i32) -> Result<i32, String> {
//     let d = divide(a, b); // Ok(2)
//     match d {
//         Ok(answer) => Ok(answer),
//         Err(err) => Err(err),
//     }
// }
//
// short hand operator for match ?
fn cal(a: i32, b: i32) -> Result<i32, String> {
    let d = divide(a, b)?; // Ok(2)
    Ok(d)
}

fn main() {
    let result = cal(4, 2); //Ok(2)
    match result {
        Ok(answer) => println!("answer is {}", answer),
        Err(error) => println!("error is {}", error),
    }
}
