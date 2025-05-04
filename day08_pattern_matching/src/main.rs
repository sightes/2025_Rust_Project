enum Animal {
    Dog(String),
    Cat(String),
    Bird,
    Unknown,
}

fn describe_animal(animal: Animal) {
    match animal {
        Animal::Dog(name) => println!("Es un perro llamado {}", name),
        Animal::Cat(name) if name == "Garfield" => println!("¡Es Garfield! El gato que odia los lunes."),
        Animal::Cat(name) => println!("Es un gato llamado {}", name),
        Animal::Bird => println!("Es un pájaro, tal vez un loro."),
        Animal::Unknown => println!("No sabemos qué animal es."),
    }
}

fn main() {
    let mascota1 = Animal::Dog(String::from("Firulais"));
    let mascota2 = Animal::Cat(String::from("Garfield"));
    let mascota3 = Animal::Bird;
    let mascota4 = Animal::Unknown;

    let zoologico = vec![mascota1, mascota2, mascota3, mascota4];

    for animal in zoologico {
        describe_animal(animal);
    }
}
