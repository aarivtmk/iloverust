// struct, constructor, impl, Self, self, &self, trait, default functions inside trait, trait bounds, static dispatch

// Instagram has User -  user_type (free_user/ premium_user) , username, followers
// instagram has Admin - user_role (accounts, marketing, service), username, followers
// common trait UserDetails -
//   common for everyone ->   find_username,
//   not common for everyone --> find_position

// create an object steve object on User struct with - user_type - free_user, steve, 25
// Admin would be Jim , accounts, jimsimons,100
#[derive(Debug)]
struct User {
    user_type: String,
    username: String,
    followers: i32,
}
struct Business {
    username: String,
    followers: i32,
    user_role: String,
}
#[derive(Debug)]
struct Admin {
    user_role: String,
    username: String,
    followers: i32,
}
trait UserDetails {
    fn myprofile(&self);
    fn get_insta_username(&self);
}

impl User {
    fn new(user_type: String, username: String, followers: i32) -> Self {
        Self {
            user_type,
            username,
            followers,
        }
    }

    fn get_followers(&self) -> i32 {
        self.followers
    }
}

impl Admin {
    fn new(user_role: String, username: String, followers: i32) -> Self {
        Self {
            user_role,
            username,
            followers,
        }
    }

    fn get_followers(&self) -> i32 {
        self.followers
    }
}

impl UserDetails for User {
    fn myprofile(&self) {
        println!("myprofile: {:?}", self)
    }
    fn get_insta_username(&self) {
        println!("USERNAME is {}", self.username);
    }
}
impl UserDetails for Admin {
    fn myprofile(&self) {
        println!("myprofile: {:?}", self)
    }
    fn get_insta_username(&self) {
        println!("USERNAME is {}", self.username);
    }
}

fn get_username<T: UserDetails>(u: &T) {
    u.get_insta_username();
}

fn main() {
    let user1 = User::new(String::from("free_user"), "steve".to_string(), 25);
    println!("user1 followers are: {}", user1.get_followers());

    user1.myprofile();
    get_username(&user1); // User::get_username(&user1);

    let user2 = Admin::new(String::from("accounts"), "jimsimons".to_string(), 225);
    println!("user2 followers are: {}", user2.get_followers());
    user2.myprofile();
    get_username(&user2); // User::get_username(&user1);

    let business_owner = Business {
        username: "jack".to_string(),
        followers: 257,
        user_role: "CEO".to_string(),
    };
    get_username(&business_owner);
}

// smart pointers - Box in memory
// vectors and box together vs vectors and Strings
// dynamic dispatch dyn, arc,mutex,refcount, axum

struct Dog; // name, age, breed
struct Cat; // name, age, breed, origin
struct Human; //name, age, gender

trait Animal {
    fn bark(&self);
}

fn animal_bark<T: Animal>(a: &T) {
    bark(a);
}
