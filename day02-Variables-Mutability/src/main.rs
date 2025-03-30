fn main() {
    // immutable variable
    let x = 5;
    println!("El valor de x es: {}", x);

    // Trying to change an immutable variable will cause an error
    // x = 6; // ❌ Does not compile

    // mutable  variable
    let mut y = 10;
    println!("El valor inicial de y es: {}", y);
    
    y = 15;
    println!("Ahora el valor de y es: {}", y);

    // shadowing
    let z = 100;
    let z = z + 50;
    println!("El valor de z después del shadowing es: {}", z);
}