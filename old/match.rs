enum PaymentStatus {
    Success(String),
    Failure(String),
    Pending(String),
}

fn main() {
    let payment_status = PaymentStatus::Pending(String::from(" payment got pending"));

    match payment_status {
        PaymentStatus::Success(r) => {
            println!("hey , thanks for payment - {}", r);
        }
        PaymentStatus::Failure(e) => {
            println!("hey , thanks for trying - {}", e);
        }
        _ => {
            println!("hey , we are looking into the issue");
        }
    }
}

// enum ZomatoDeliveryStatus - Pickup,Transit,Delivered
