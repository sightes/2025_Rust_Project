📚 Day 03 – Data Types & Functions in Rust

Welcome to Day 3 of learning Rust! Today, we’re diving into two core building blocks of any programming language: data types and functions. 🦀✨

⸻

🧠 What You’ll Learn
	•	Scalar vs. Compound data types
	•	Common Rust types: i32, f64, bool, char, tuple, array
	•	How to define and call functions
	•	Function parameters and return types

⸻

🧮 Basic Data Types in Rust

🧱 Scalar Types

These represent a single value:
	•	i32 → 32-bit signed integer (default)
	•	f64 → 64-bit floating point (default)
	•	bool → true or false
	•	char → single Unicode character

```rust
fn main() {
    let age: i32 = 30;
    let temp: f64 = 36.6;
    let is_active: bool = true;
    let heart = '❤';

    println!("Age: {}, Temp: {}, Active: {}, Symbol: {}", age, temp, is_active, heart);
}
```
🧳 Compound Types

tuple

```rust
let person: (&str, i32) = ("Alice", 28);
println!("Name: {}, Age: {}", person.0, person.1);
```

array

```rust
let numbers: [i32; 4] = [1, 2, 3, 4];
println!("First number: {}", numbers[0]);
```


🔧 Functions in Rust

Rust functions start with the fn keyword and must specify types for all parameters.

```rust
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
```