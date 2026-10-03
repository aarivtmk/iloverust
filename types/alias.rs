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




fn main(){
    let a = 10;
    let arr = [1,2,4,6,7,8444,.... 1billion ]
    let newarray:[i32, 1_000_000_000] = []
    if (vec.len()%2 == 0){
        println!("even length")
    } // O(1)

    for (int i = 0; i<vec.len();i++){
        newarray[i] = arr[i]
    } // O(n)
}

O(n)
