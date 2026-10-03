use std::collections::BTreeSet;
fn main() {
    let mut users = BTreeSet::new();

    users.insert(111);
    users.insert(2);
    users.insert(7);
    users.insert(7);
    println!("user1 is {:?}", users);

    for user in &users {
        println!("{}", user);
    }
}
