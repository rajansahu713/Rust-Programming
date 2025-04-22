## References
- In Rust, references are a way to access data without taking ownership of it. They are denoted by the `&` symbol.

- References are used to borrow data, allowing multiple parts of your code to access the same data without needing to create copies. This is particularly useful for large data structures or when you want to share data between functions without transferring ownership.


- References can be either mutable or immutable. Immutable references allow you to read data, while mutable references allow you to modify it.
- Rust enforces strict rules around references to ensure memory safety and prevent data races.

## The rules of references are as follows
- You can have multiple immutable references to the same data.
- You can have one mutable reference to the same data. and you cant have mutuable reference and immutable reference to the same data at the same time.
- References must always be valid. This means that the data they point to must not be dropped while the reference is still in use.



### Example 1 (can create mutable references and immutable references at the same time but not for same variable):
```rust
fn main() {
    let x = 5;
    // Immutable reference to x
    let y = &x;

    let mut j = 7
    // Mutable reference to j
    let z: &mut i32 = &mut j;
    println!("x: {}, y: {}, z: {}", x, y, z); 

}
```

### Example 2 (can create multiple immutable references to the same data):
```rust
fn main() {
    let x = 5;
    // Immutable reference to x
    let y = &x;
    let z = &x; // Another immutable reference to x

    println!("x: {}, y: {}, z: {}", x, y, z); 
}
```