fn change_username(n: &str) -> &str {
    n
}

fn main() {
    let mut username = String::from("sree");
    println!("before update username is {}", username);
    let result = change_username("medha");
    username = result.to_string();
    println!("after update username is {}", username);
}
