use std::collections::BTreeMap;
fn main() {
    let mut users = BTreeMap::new();
    users.insert(101, "Aariv");
    users.insert(102, "Steve");
    users.insert(103, "Phunsuk");
    println!("user are {:?}", users);
}

/*
 *
 *              STACK
┌──────────────┐
│    users     │
│              │
│ ptr ─────────┼──────────────┐
│ len          │              │
│ capacity     │              │
└──────────────┘              │
                          ↓
                      HEAP TABLE
        ┌──────────────────────────┐
        │ CONTROL                  │
        │                          │
        │ [C][C][C][C][C][C]       │
        │                          │
        │ KEY/VALUE DATA           │
        │                          │
        │ 101 → Aariv              │
        │ 102 → Koel               │
        │ 103 → Phunsuk            │
        └──────────────────────────┘
* HashMap
    ↓
HASH
    ↓
"WHERE SHOULD THIS KEY BE?"
    ↓
O(1) average


BTreeMap
    ↓
SORTED TREE
    ↓
"WHICH BRANCH SHOULD I TAKE?"
    ↓
O(log n)
*
*
*
 */
