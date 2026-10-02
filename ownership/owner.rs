fn consume(st: &mut String) -> (&String, usize) {
    st.clear();
    st.push_str("Steve");
    (st, st.len())
}

fn main() {
    let mut s = String::from("hello");
    let (username, user_len) = consume(&mut s);
    println!("{:?},{}", username, user_len); // ❌
}

// send username to the change_username function
// return - new username & also len of the new username in the tuple
