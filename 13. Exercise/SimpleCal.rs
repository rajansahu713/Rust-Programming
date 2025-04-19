fn main() {
    println!("-------------------Simple Calculator-----------------");
    let a: f32 = 30.0;
    let b: f32 = 4.0;

    let mul : f32 = a * b;
    let div : f32 = a / b;
    let add : f32 = a + b;
    let sub : f32 = a - b;

    println!("Sum of a and b is: {}", add);
    println!("Difference of a and b is: {}", sub);
    println!("Product of a and b is: {}", mul);
    println!("Division of a and b is: {}", div);
}