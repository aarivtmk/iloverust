use std::collections::BTreeMap;
fn main() {
    let mut users = BTreeMap::new();
    users.insert(101, "Aariv");
    users.insert(102, "Steve");
    users.insert(103, "Phunsuk");
    println!("user are {:?}", users);
}
