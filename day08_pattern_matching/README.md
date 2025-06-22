# Day 8: Pattern Matching in Rust 🦀✨

This example demonstrates how to use `match` expressions in Rust with `enum` types, including conditional matching and destructuring.

## Overview 🧠

We define an `enum` called `Animal` with several variants:

```rust
enum Animal {
    Dog(String),
    Cat(String),
    Bird,
    Unknown,
}
````

Each variant may contain data (like a String name) or no data at all.

Matching Variants 🎯

The function describe_animal uses match to:
	•	Destructure and extract values.
	•	Handle conditional matches (e.g., when the cat is “Garfield”).
	•	Provide custom responses for each type of animal.

```rust
fn describe_animal(animal: Animal) {
    match animal {
        Animal::Dog(name) => println!("It's a dog named {}", name),
        Animal::Cat(name) if name == "Garfield" => println!("It's *Garfield*! The cat who hates Mondays."),
        Animal::Cat(name) => println!("It's a cat named {}", name),
        Animal::Bird => println!("It's a bird, maybe a parrot."),
        Animal::Unknown => println!("Unknown animal."),
    }
}
```
