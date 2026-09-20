fn sub(n1: i32, n2: i32) -> Result<i32, String> {
    if n2 != 0 && n1 != 0 {
        Ok(n1 - n2) // Ok(2)
    } else {
        Err(String::from("can't have a  0"))
    }
}
