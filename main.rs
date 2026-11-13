// fn main() {
//     let a: i64 = 25;
//     let b: i64 = 35;
//
//     let result = a + b;
//
//     println!("{result}");
// }
//
fn main() {
    let data = vec![42u8; 1024 * 1024];

    println!("Address: {:p}", data.as_ptr());

    std::thread::sleep(std::time::Duration::from_secs(30));
}
