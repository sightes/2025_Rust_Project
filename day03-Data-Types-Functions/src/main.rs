fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon means this is the return value
}

fn main() {
    greet("Rustacean");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
}