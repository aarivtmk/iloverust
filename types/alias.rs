fn main() {
    type UserId = u64;
    let id: UserId = 32;
    println!("user id is {}", id);
    type User = (u64, String, bool);

    let users: Vec<User> = Vec::new();
    let users = vec![
        (1, String::from("Aariv"), true),
        (2, String::from("Bob"), false),
    ];
    println!("user id is {:?}", users[1].1);
}
