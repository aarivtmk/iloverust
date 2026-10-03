// linked list

use std::collections::LinkedList;

fn main() {
    let mut routes = LinkedList::new();
    routes.push_back("Shimla");
    routes.push_back("Spiti");
    routes.push_back("Manali");
    routes.push_front("Chandigarh");
    println!("first element is {:?}", routes.front());
    if let Some(ele) = routes.front() {
        println!("routes are {:?}", ele);
    }
}

/*
* |                 | `Vec`          | `LinkedList`   |
| --------------- | -------------- | -------------- |
| Memory          | contiguous     | separate nodes |
| Index access    | O(1)           | O(n)           |
| Push back       | O(1) amortized | O(1)           |
| Push front      | O(n)           | O(1)           |
| Cache friendly  | ✅              | ❌              |
| Memory overhead | low            | higher         |


STACK
┌─────────────────┐
routes         │ head ────────────┼────────────┐
│ tail ────────┐   │            │
│ len = 4     │   │            │
└──────────────│───│────────────┘
        │   │
        │   ↓
        │  Node 10
        │
        ↓
      Node 30

HEAP

┌─────────────────────────┐
│ Node                     │
│ value = 10               │
│ prev                     │
│ next ────────────────────┼────┐
└─────────────────────────┘    │
                   ↓
          ┌─────────────────────────┐
          │ Node                    │
          │ value = 20              │
          │ prev                    │
          │ next ───────────────────┼────┐
          └─────────────────────────┘    │
                                        ↓
                               ┌─────────────────┐
                               │ Node            │
                               │ value = 30      │
                               │ prev            │
                               │ next = NULL     │
                               └─────────────────┘
*
*
 */
