## Enums
- Enums are a way to define a set of named values. They are useful for representing a fixed set of related constants, such as days of the week, colors, or states. Enums can make your code more readable and maintainable by giving meaningful names to these constants.


1. Eample of Enum
```rust
enum Color {
    Red,
    Green,
    Blue,
}
fn main() {
    let color = Color::Red;
    match color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }
}
```

2. Example of Enum with data
```rust
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
}
fn main() {
    let shape = Shape::Circle(5.0);
    match shape {
        Shape::Circle(radius) => println!("The radius of the circle is {}", radius),
        Shape::Rectangle(width, height) => println!("The width and height of the rectangle are {} and {}", width, height),
    }
}
```

3. Example of Enum with methods
```rust
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
}
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}
fn main() {
    let shape = Shape::Circle(5.0);
    println!("The area of the shape is {}", shape.area());
}
```
