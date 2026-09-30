use std::collections::BTreeSet;
fn main() {
    let mut users = BTreeSet::new();

    users.insert("aariv");
    users.insert("101");
    users.insert("aariv");
    users.insert("steve");
    println!("user1 is {:?}", users);

    for user in &users {
        println!("{}", user);
    }
}
