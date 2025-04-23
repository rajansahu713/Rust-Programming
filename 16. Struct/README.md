## Struct
Structs are used to create custom data types in Rust. They allow you to group related data together.


Example

1. Basic Struct
```rust
struct Person {
    name: String,
    age: u32,
}
fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Name: {}, Age: {}", person.name, person.age);
}
```
2. Struct with Methods
```rust
struct Rectangle {
    width: u32,
    height: u32,
    pl Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    fn main() {
        let rect = Rectangle {
            width: 10,
            height: 5,
        };
        println!("Area of rectangle: {}", rect.area());
    }
}
```
