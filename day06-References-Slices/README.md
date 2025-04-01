📚 What Is a Reference Slice in Rust?

A slice is a reference to a part of a collection, like a String or an array, without taking ownership of the data.

🔍 Why Use Slices?
	•	To borrow part of a collection without copying it.
	•	Useful when you want to read data without taking ownership.
	•	Helps in efficient memory use and safe access.

```rust
fn main() {
    let message = String::from("Hello, Rustaceans!");

    let hello = &message[0..5]; // Slice: "Hello"
    let rustaceans = &message[7..]; // Slice: "Rustaceans!"

    println!("First word: {}", hello);
    println!("Rest: {}", rustaceans);

    // Slices also work with arrays
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4]; // Slice: [20, 30, 40]

    println!("Middle numbers: {:?}", middle);
}

```


✨ Key Points
	•	&[T] is a slice of a generic array.
	•	&str is a slice of a string (String).
	•	Slices are non-owning references – they borrow, not own.
	•	You can use slices to avoid unnecessary copying and keep your code efficient and clean.

⸻
