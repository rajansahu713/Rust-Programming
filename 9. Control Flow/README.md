## Flow Control

Flow control is a fundamental concept in programming that allows you to dictate the order in which statements are executed. In Python, flow control is primarily managed through conditional statements and loops. This section will cover the following topics:
- **Conditional Statements**: These are used to perform different actions based on different conditions. The main conditional statements in Python are `if`, `elif`, and `else`.


## Conditional Statements
1. Write a program to check if a number is even or odd.
```rust
fn main() {
    let number = 5;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
```
2. write a program to check you Grades.
```rust
fn main() {
    let grade = 85;
    if grade >= 90 {
        println!("A");
    } else if grade >= 80 {
        println!("B");
    } else if grade >= 70 {
        println!("C");
    } else if grade >= 60 {
        println!("D");
    } else {
        println!("F");
    }
}
```
3. How to use conditional statements as expression.
```rust
fn main() {
    let age = 18;
    let is_adult = if age >= 18 { true } else { false };
    println!("Age: {} is_adult  {}",age, is_adult);
}
```