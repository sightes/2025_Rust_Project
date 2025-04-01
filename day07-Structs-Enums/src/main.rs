enum PaymentMethod {
    CreditCard(String),
    Paypal(String),
    Bitcoin,
}

fn process_payment(method: PaymentMethod) {
    match method {
        PaymentMethod::CreditCard(card) => println!("💳 Paid with credit card: {}", card),
        PaymentMethod::Paypal(email) => println!("📧 Paid with PayPal: {}", email),
        PaymentMethod::Bitcoin => println!("₿ Paid with Bitcoin"),
    }
}

fn main() {
    let method1 = PaymentMethod::CreditCard(String::from("1234-5678-9012"));
    let method2 = PaymentMethod::Bitcoin;
    let method3 = PaymentMethod::Paypal(String::from("user@example.com")); // 👈 using Paypal

    process_payment(method1);
    process_payment(method2);
    process_payment(method3); // 👈 don't forget to process it!
}