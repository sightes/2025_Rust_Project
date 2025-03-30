# Day 02 - Variables & Mutability 🦀

Welcome to Day 2 of learning Rust! Today we're exploring one of Rust's foundational features: **variables** and their **mutability**.

---

## 📘 Concepts

### 🔹 Variable

A variable in Rust is a named memory location used to store a value. You create one with the `let` keyword.

```rust
let age = 30;

🔹 Immutability

By default, variables in Rust are immutable, which means their values cannot be changed after they’re bound.

let name = "Alice";
// name = "Bob"; ❌ Error: cannot assign twice to immutable variable

This design promotes safety and predictability.

🔹 Mutability

To make a variable mutable, use the mut keyword:

let mut counter = 0;
counter = 1; // ✅ Works!

Mutable variables can have their values changed, but you must be explicit about it—Rust won’t allow silent changes.

⸻

🧠 Why Does This Matter?

Rust encourages immutability to reduce bugs and promote thread safety. When you make a variable mutable, you’re telling the compiler and readers of your code: “this might change.”

This simple rule helps prevent unintended side effects, especially in concurrent programming.

⸻
🧪 Examples

Immutable Variable

fn main() {
    let score = 42;
    println!("Score: {}", score);

    // score = 50; // ❌ Error: cannot assign twice to immutable variable
}

Mutable Variable

fn main() {
    let mut score = 42;
    println!("Score: {}", score);

    score = 50; // ✅ Works fine
    println!("Updated Score: {}", score);
}