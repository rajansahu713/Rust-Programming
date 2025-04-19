## Ownership

Ownership is a key concept in Rust that determines how memory is managed. In Rust, each value has a single owner, and when the owner goes out of scope, the value is dropped (deallocated). This ensures memory safety without needing a garbage collector. In simple terms a variable is monogamy, it can only be owned by one variable at a time. When you assign a variable to another variable, the ownership is transferred, and the original variable can no longer be used.

Ownership rules:
- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
- Ownership can be transferred (moved) from one variable to another.
- Ownership can be borrowed (temporarily accessed) using references.
- Borrowing can be mutable or immutable, but not both at the same time.
- References must always be valid, meaning they cannot outlive the data they point to.
- The Rust compiler checks ownership and borrowing rules at compile time, ensuring memory safety without runtime overhead.
- Ownership is a compile-time concept, meaning it is checked at compile time and does not incur runtime overhead.

Why is ownership important?
- It prevents memory leaks and dangling pointers.
- It allows for efficient memory management without runtime overhead.
- It enables safe concurrency by ensuring that data is not shared between threads without proper synchronization.
- It provides a clear and explicit way to manage resources, making the code easier to understand and maintain.

## Ownership Imporatnce in Rust

|Type   |	Is it Copy? |	Can ownership be transferred?   |	Explanation |
|----------------|----------------|-------------------------------|------------------|
|Integer (i32)|	✅ Yes  |	❌ No (it copies instead)   |	Integers implement the Copy trait, so assigning or passing them just copies the value.
|String | ❌ No |	✅ Yes |	String is heap-allocated and does not implement Copy, so ownership moves on assignment or passing.
|Array  ([i32; 3])  |	✅ Yes |(if all elements are Copy and size is fixed)	❌ No (it copies)   |	Fixed-size arrays of Copy types (like [i32; 3]) are themselves Copy. So assigning or passing copies them.
|Vector (Vec<i32>)  |	❌ No |	✅ Yes	|Vec is heap-allocated and does not implement Copy, so ownership transfers on assignment.
|Struct |	❌ Depends |	✅ Usually Yes  |	If all fields are Copy and #[derive(Copy)] is added, it can be Copy. Otherwise, ownership is transferred.
|Enum   |	❌ Depends  |	✅ Usually Yes |	Simple enums (like enum MyEnum { A, B }) can be Copy. If any variant contains non-Copy data (like String), it's not Copy.



While going to through the ownership concept, a question will raised why string ownershit is transfer but not in case of integer.

Lets me answer this question with an example.

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
}
```

In this example, `x` is an integer, which is a Copy type. When we assign `x` to `y`, the value of `x` is copied to `y`, and both variables can be used independently. And the size  of integer is fixed and known at compile time, so the compiler can optimize the copying process.

and variable `x` hold memory1 and variable `y` hold memory2.

```rust
fn main() {
    let s = String::from("Hello");
    let s2 = s1;
    println!("s1: {}, s2: {}", s1, s2);
}
```

In this example, `s` is a String, which is not a Copy type. When we assign `s` to `s2`, the value of `s` is moved to `s2`, and `s` is no longer valid. The ownership of the string is transferred from `s` to `s2`.
When we run the above code, we will get an error like this:

```bash
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:4:36
  |> 4 |     println!("s1: {}, s2: {}", s, s2);
  |                                    ^ value borrowed here after move
  |
  = note: move occurs because `s` has type `String`, which does not implement the `Copy` trait
  `
```

This is because `String` is a heap-allocated type, and the ownership of the memory is transferred when we assign it to another variable. The compiler ensures that we do not access the original variable after the ownership has been moved, preventing potential memory safety issues.


### Why string goes on heap?

* Because the size of a String is not known at compile time.

Example:
```rust
let a = String::from("hello");
let b = String::from("a really long sentence written by Rajan");
```
2. At compile time, Rust doesn’t know how long the string will be — especially if it comes from:
    * User input
    * API response
    * File content

3. so, it uses the heap, which is flexible and can grow as needed.

🔧 What’s actually on the stack?
When you write:

```rust
let name = String::from("Rajan");
```
Rust stores three things on the stack:

* Pointer to the actual string data on the heap
* cLength of the string
* Capacity (how much memory is allocated)

So this:
```bash
name -> [ptr -> 🧠 heap], len = 5, cap = 5
```
Heap:
🧠: "Rajan"
💡 In contrast, literals like &str are different:
```rust
let s = "Rajan"; // &'static str
```
This is a string slice
It's statically allocated (compiled into the binary)
No heap allocation needed
It’s fixed size and points to read-only data


