// build a calculator
fn add(m: i32, n: i32) -> Result<i32, String> {
    if m != 0 && n != 0 {
        Ok(n + m)
    } else {
        Err("Error: Please enter both numbers".to_string())
    }
}

fn cal(op: &str, a: i32, b: i32) -> Result<i32, String> {
    let result = if op == "add" {
        add(a, b)?
    } else {
        return Err("unknown operation".to_string());
    };
    Ok(result)
}

fn main() {
    // match cal("add", 4, 3) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(error) => println!("Error: {}", error),
    // }
    // if let pattern = value
    if let Ok(result) = cal("add", 8, 3) {
        println!("Result {}", result);
    }

    let result = cal("add", 0, 4);
    match result {
        Ok(answer) => println!("Result {}", answer),
        Err(_) => {}
    }
}
