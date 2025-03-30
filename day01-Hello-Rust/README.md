# Hello Rust 👋🦀

Welcome to **Hello Rust**, a basic starter project to get familiar with the Rust programming language! This simple example prints `Hello, Rust!` to the console and introduces you to how Rust projects are structured using Cargo (Rust’s package manager and build tool).

---

## 🧠 What is Rust?

**Rust** is a modern systems programming language developed by **Mozilla Research** and officially released in **2010**. It was designed to provide:
- **Memory safety** without needing a garbage collector.
- **High performance**, rivaling C and C++.
- **Fearless concurrency**, allowing developers to write multithreaded code more safely.

Rust has been voted the [“most loved programming language” on Stack Overflow](https://insights.stackoverflow.com/survey) for several years in a row, and it's used by companies like Microsoft, Amazon, Google, and Dropbox.

---

## 📁 Project Structure

When you run `cargo new hello-rust`, Cargo creates the following structure:

hello-rust/
├── Cargo.toml       # Project metadata and dependencies
└── src/
└── main.rs      # Main entry point for the application

- `Cargo.toml`: Like `package.json` or `pom.xml`, this file defines your project’s dependencies, version, and name.
- `src/main.rs`: The main source file. Rust begins execution here by calling the `main()` function.

---

## 🚀 Running the Project

To build and run this Rust project, make sure you have [Rust installed](https://www.rust-lang.org/tools/install), then:

```bash
cd hello-rust
cargo run


You should see the following output:

Hello, Rust!