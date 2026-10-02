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
    queue.push_back(10);
    queue.push_back(20);
    println!("queue is {:?}", queue);
    queue.push_front(40);
    println!("queue is {:?}", queue);
    queue.pop_back();
    println!("queue is {:?}", queue);
    queue.pop_front();
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
