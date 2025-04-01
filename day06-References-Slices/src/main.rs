fn main() {
    let message = String::from("Hello, Rustaceans!");

    let hello = &message[0..5]; // Slice de los primeros 5 caracteres
    let rustaceans = &message[7..]; // Slice desde el índice 7 hasta el final

    println!("First word: {}", hello);
    println!("Rest: {}", rustaceans);

    // También funciona con arrays
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4]; // [20, 30, 40]

    println!("Middle numbers: {:?}", middle);
}