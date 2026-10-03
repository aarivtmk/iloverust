/*
*
*
* Vec:
push → usually at the END

VecDeque:
push → END or FRONT
pop  → END or FRONT
*/
use std::collections::VecDeque;
fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(10); // O(1)
    queue.push_back(20); // O(1)
    println!("queue is {:?}", queue);
    queue.push_front(40); // O(1)
    println!("queue is {:?}", queue);
    queue.pop_back(); O(1)
    println!("queue is {:?}", queue);
    queue.pop_front(); O(1)
    println!("queue is {:?}", queue);
    println!("first element is {:?}", queue[0]);
}

/*
*
* Why not use Vec?

With a Vec:
vec.insert(0, 5);

Rust has to move all existing elements:
[10][20][30][40]
      ↓
[5][10][20][30][40]
    ↑   ↑   ↑   ↑
    everything shifted
That's O(n).
 */
