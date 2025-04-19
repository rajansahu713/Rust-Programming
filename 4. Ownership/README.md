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