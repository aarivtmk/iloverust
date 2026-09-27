// sign the transaction

static PRIVATE_KEY: &str = "xyz&12345";
static PUBLIC_KEY: &str = "123pub";
fn sign_txn<'a>(pvt_key: &'a str, message: &'a str, signature: &'a mut String) -> &'a str {
    signature.push_str(pvt_key);
    signature.push_str(message);
    signature
}

fn verify_txn() -> bool {
    true
}
fn main() {
    let mut signature = String::from("");
    // println!(
    //     "signature : {}",
    //     sign_txn(&PRIVATE_KEY, "hello", &mut signature)
    // );
    let message = "hello";
    signature = sign_txn(&PRIVATE_KEY, , &mut signature);
    verify(&PUBLIC_KEY, signature)
}
