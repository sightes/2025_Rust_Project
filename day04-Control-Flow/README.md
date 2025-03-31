# 📆 Day 03 - Control Flow in Rust (if, match, enums & if let)

Rust allows rich control flow using `if`, `match`, and pattern matching with enums. These tools help you write clear, safe, and expressive logic.

---

## 🎭 Pattern Matching with `enum` and `match`

`match` really shines when used with `enums`. Here's an example with a custom enum to represent weather conditions.

### ✅ Example:

```rust
enum Weather {
    Sunny,
    Rainy,
    Windy(u8), // Wind speed in km/h
    Snowy { inches: u8 },
}

fn main() {
    let today = Weather::Windy(35);

    match today {
        Weather::Sunny => println!("Wear sunglasses 😎"),
        Weather::Rainy => println!("Take an umbrella ☔"),
        Weather::Windy(speed) if speed > 30 => println!("Too windy! Stay indoors 🌀"),
        Weather::Windy(speed) => println!("Mild wind of {} km/h", speed),
        Weather::Snowy { inches } if inches > 10 => println!("Snow day! Build a snowman ⛄"),
        Weather::Snowy { inches } => println!("Light snow: {} inches ❄", inches),
    }
}


🔎 if let - Pattern Matching Lite

if let is useful when you only care about one pattern and want a simpler alternative to match.

✅ Example:
```rust
fn main() {
    let config_max = Some(100);

    if let Some(max) = config_max {
        println!("The max is set to {}", max);
    } else {
        println!("No max value set.");
    }
}
```


This is equivalent to:

```rust
match config_max {
    Some(max) => println!("The max is set to {}", max),
    None => println!("No max value set."),
}
```