mod sbi;
use sbi::{interest::calculate_interest, reward::get_rewards};
mod math;
use math::User;
fn main() {
    let u = User {
        name: "steve".to_string(),
        age: 22,
    };
    println!("u is {:?}", u);
    // let answer = calculate_interest();
    let answer = get_rewards();
    println!("{:?}", answer);
}
