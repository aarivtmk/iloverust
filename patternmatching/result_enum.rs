// error handling - Result and OPtion
// // result - Ok or Err
// Option - Some or None
//
//
//

// admin panle code - if user id = 123, then allow else error
// Result<Ok,Err>
fn login_admin(u: i32) -> Result<String, String> {
    if u == 123 {
        Ok("you are invited to adminpanel".to_string())
    } else {
        Err("Access Denied".to_string())
    }
}

fn fetch_posts(subscribed: &str) -> Option<String> {
    if subscribed == "sports" {
        Some("Hey , here are news in sports".to_string())
    } else {
        None
    }
}
fn main() {
    let user_id: i32 = 123;

    if let Ok(message) = login_admin(user_id) {
        println!("{}", message);
    }

    // match login_admin(user_id) {
    //     Ok(message) => println!("{}", message),
    //     Err(error) => println!("{}", error),
    // }

    // match fetch_posts("movies") {
    //     Some(message) => println!("{}", message),
    //     None => println!("No new feed"),
    // }
}
