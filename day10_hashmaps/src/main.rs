use std::collections::HashMap;

fn main() {
    // Crear un nuevo HashMap
    let mut scores = HashMap::new();

    // Insertar valores
    scores.insert("Alice", 10);
    scores.insert("Bob", 15);
    scores.insert("Carol", 20);

    // Acceder a un valor
    if let Some(score) = scores.get("Bob") {
        println!("Bob tiene {} puntos.", score);
    }

    // Actualizar un valor
    scores.insert("Bob", 18);

    // Insertar solo si no existe
    scores.entry("Dave").or_insert(12);

    // Recorrer todos los pares clave-valor
    println!("Puntajes actuales:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Eliminar una entrada
    scores.remove("Alice");
}