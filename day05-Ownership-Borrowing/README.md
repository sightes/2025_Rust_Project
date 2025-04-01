📦 Day 4 - Ownership & Borrowing in Rust

Rust’s Ownership model is one of its most unique and powerful features, enabling memory safety without a garbage collector. To master Rust, understanding how ownership and borrowing work is essential.

⸻

🌱 Ownership

In Rust, each value has a single owner — a variable that holds the value. When ownership is transferred (moved), the original variable becomes invalid.
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moves from s1 to s2

    // println!("{}", s1); // ❌ Error! s1 is no longer valid
    println!("{}", s2); // ✅ This works
}

```
🔁 Borrowing

Rust allows variables to borrow values through references, either mutable or immutable.

Immutable Borrowing (&T)

You can have multiple immutable references:
```rust
fn main() {
    let s = String::from("hello");
    print_string(&s);
    println!("{}", s); // ✅ Still valid, because we only borrowed it immutably
}

fn print_string(text: &String) {
    println!("{}", text);
}
```

Mutable Borrowing (&mut T)

You can have only one mutable reference at a time:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // ✅ "hello world"
}

fn change(text: &mut String) {
    text.push_str(" world");
}
```
🎯 Why It Matters

Rust’s ownership system:
	•	Eliminates data races at compile time.
	•	Ensures memory safety without a runtime cost.
	•	Encourages writing efficient and correct code.