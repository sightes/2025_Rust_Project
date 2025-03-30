use std::io;

fn main() {
    println!("=== Calculadora Básica en Rust ===");

    loop {
        // Leer el primer número
        let num1 = match read_number("Introduce el primer número:") {
            Some(n) => n,
            None => continue,
        };

        // Leer el segundo número
        let num2 = match read_number("Introduce el segundo número:") {
            Some(n) => n,
            None => continue,
        };

        // Leer el operador
        println!("Introduce la operación (+, -, *, /):");
        let mut operator = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Error al leer el operador");

        let operator = operator.trim();

        // Calcular el resultado
        let result = match operator {
            "+" => Some(num1 + num2),
            "-" => Some(num1 - num2),
            "*" => Some(num1 * num2),
            "/" => {
                if num2 != 0.0 {
                    Some(num1 / num2)
                } else {
                    println!("Error: No se puede dividir entre cero.");
                    None
                }
            }
            _ => {
                println!("Error: Operador no válido.");
                None
            }
        };

        // Mostrar el resultado si es válido
        if let Some(res) = result {
            println!("El resultado de {} {} {} es: {}", num1, operator, num2, res);
        }

        // Preguntar si el usuario quiere continuar
        println!("¿Quieres realizar otra operación? (s/n):");
        let mut continue_input = String::new();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("Error al leer entrada");

        if continue_input.trim().to_lowercase() != "s" {
            break;
        }
    }

    println!("¡Gracias por usar la calculadora!");
}

// Función para leer un número de la entrada estándar
fn read_number(prompt: &str) -> Option<f64> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada");
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Error: Por favor introduce un número válido.");
            None
        }
    }
}