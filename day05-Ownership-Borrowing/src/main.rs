fn main() {
    let mut book = String::from("The Rust Book");

    // Immutable borrow
    print_book_info(&book); 

    // Mutable borrow
    update_book_title(&mut book); 

    // Ownership moved to donate_book
    donate_book(book); 

    // println!("{}", book); // ❌ Error: `book` was moved
}

fn print_book_info(title: &String) {
    println!("📖 Current title: {}", title); // Immutable borrow
}

fn update_book_title(title: &mut String) {
    title.push_str(" - Second Edition");
    println!("🔧 Updated title: {}", title); // Mutable borrow
}

fn donate_book(title: String) {
    println!("🎁 Donated: {}", title); // Ownership taken
}