## Pattern Matching
- Pattern matching is a powerful feature in Rust that allows you to match a value against a set of patterns and execute different code blocks based on the matched pattern. It is commonly used in Rust for control flow, data processing, and error handling.    

Example

1. Basic Pattern Matching
```rust
fn main() {
    let number = 5;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Not One, Two, or Three"),
    }
}
```

2. Pattern Matching with Multiple Patterns
```rust
fn main() {
    let number = 2;
    match number {
        1 | 2 => println!("One or Two"),
        3 => println!("Three"),
        _ => println!("Not One, Two, or Three"),
    }
}
```
3. Pattern Matching with Ranges
```rust
fn main() {
    let number = 5;
    match number {
        1..=5 => println!("Between 1 and 5"),
        _ => println!("Not between 1 and 5"),
    }

}
```
4. Pattern Matching with Structs
```rust
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let point = Point { x: 3, y: 4 };
    match point {
        Point { x, y } => println!("Point is at ({}, {})", x, y),
    }
}
```
5. Pattern Matching with Option
```rust
fn main() {
    let number = Some(5);
    match number {
        Some(n) if n > 0 => println!("Positive number: {}", n),
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }
}
```
6. Pattern Matching with Enums
```rust
enum Color {
    Red,
    Green,
    Blue,
}
fn main() {
    let color = Color::Green;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
```
