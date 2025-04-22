### Constants
- Constants are used to define fixed values that do not change during the execution of a program. 
- It store value in data segment of memory that is read-only and cannot be modified at runtime.
- you can decleare constant in Global scope.

```rust
const PI = 3.14;
fn main() {
    println!("The value of PI is: {}", PI);
}
```
- you may riase question why contant are store in data segment of memory.
- In rust, constants are stored in the data segment of memory, which is a section of memory that is used to store data that is not modified during the execution of a program.

- This is because constants are defined at compile time and their values are known before the program is run.