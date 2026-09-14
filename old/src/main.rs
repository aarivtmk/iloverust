enum SessionState {
    Anonymous,
    Pending {
        magic_link: String,
        time_stampd: u64,
    },
    Active {
        user_id: String,
        session_id: String,
    },
    Expired {
        reason: String,
    },
}

fn describe(s: &SessionState) -> String {
    // if let SessionState::Anonymous = s {
    //     return "anonymous".to_string();
    // }
    if let SessionState::Active {
        user_id,
        session_id,
    } = s
    {
        if user_id == "123" && session_id == "abc" {
            return "active".to_string();
        }
    }
    "something else".to_string()
}
pub fn main() {
    // let result = describe(&SessionState::Anonymous);
    let result1 = describe(&SessionState::Active {
        user_id: "123".to_string(),
        session_id: "abc".to_string(),
    });
    println!("OUTPUT : {}", result1);
}
