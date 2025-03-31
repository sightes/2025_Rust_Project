

fn main() {
    let config_max = Some(100);

    match config_max {
        Some(max) => println!("The max is set to {}", max),
        None => println!("No max value set."),
    };

    if let Some(max) = config_max {
        println!("The max is set to {}", max);
    } else {
        println!("No max value set.");
    }
}