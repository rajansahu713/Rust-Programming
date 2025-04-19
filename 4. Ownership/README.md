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