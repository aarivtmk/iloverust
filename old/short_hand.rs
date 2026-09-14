fn divide(n: i32) -> Result<i32, String> {
    if n == 0 {
        Err("Cant divide by 0".to_string())
    } else {
        Ok(n / 2)
    }
}

// short hand operator ?

fn calc(n: i32) -> Result<i32, String> {
    let result = divide(n)?;
    // match result {
    //     Ok(result) => Ok(result),
    //     Err(error) => Err(error),
    // }
    Ok(result)
}

fn main() {
    match calc(2) {
        Ok(result) => println!("result is {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

// build a calculator  add & sub
// calc ("add",2,4)
