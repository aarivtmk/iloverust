use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(101, "Aariv");
    users.insert(102, "Wangdu");
    users.insert(103, "Phunsuk");

    users.insert(102, "Koel");

    println!("{:?}", users.get(&102));
}

/*
* STACK
────────────────────────────

users
┌─────────────────────────┐
│ table pointer ───────────┼──────────────┐
│ len = 3                 │              │
│ capacity / table info   │              │
│ hash state / metadata   │              │
└─────────────────────────┘              │
                                         ↓
                                      HEAP
                              ┌──────────────────┐
                              │ HashMap table    │
                              │                  │
                              │ 101 → "Aariv"   │
                              │ 102 → "Koel"     │
                              │ 103 → "Phunsuk"  │
                              │                  │
                              │ empty capacity  │
                              │ control data    │
                              └──────────────────┘
*
*
 */
