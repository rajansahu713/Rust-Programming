## Compound Types

Compound types are types that can group multiple values into one. Rust has two primitive compound types: tuples and arrays.

### Tuples
- A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- tuple is hetorogeneous, meaning that it can contain values of different types.

```rust
let tup:(i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // destructuring
println!("The value of y is: {}", y); // prints 6.4
```

### Arrays
- An array is a collection of values of the same type.
- Arrays have a fixed length, which is determined when the array is created.
- Arrays are stored in contiguous memory, which means that they can be accessed quickly.
- If you try to access the index which is out of bounds of the array, it will throw a panic at runtime not in the compiler time.


