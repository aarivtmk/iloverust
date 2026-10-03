fn main() {
    let mut numbers = vec![10, 20, 30, 40, 50];
    numbers.push(100); // O(n)
    numbers.pop(); // O(n)
    numbers.insert(0, 66); // O(n)
                           // [66,10, 20, 30, 40, 50]
    println!("slide is {:?}", numbers);
}

// Online C compiler to run C program online
