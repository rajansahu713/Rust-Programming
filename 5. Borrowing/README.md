## Borrowing
Borrowing is a mechanism in Rust that allows you to temporarily access a value without transferring ownership. This is done using references. There are two types of borrowing: mutable and immutable.
- **Immutable Borrowing**: You can borrow a value immutably using `&`. This allows you to read the value without modifying it. Multiple immutable borrows are allowed at the same time.
- **Mutable Borrowing**: You can borrow a value mutably using `&mut`. This allows you to modify the value. However, only one mutable borrow can exist at a time.


### Rules of Borrowing
* While a mutable reference exists, you cannot access the original variable.
* If you have a mutable reference, you cannot have any other references to the same variable.
* If you have an immutable reference, you can have multiple immutable references.
* You cannot have a mutable reference and an immutable reference to the same variable at the same time.
* References must always be valid. This means that the data they point to must not be dropped while the reference is still in use.

### Example for 1st rule:
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    println!("{}", x); // Error: cannot borrow `x` as mutable because it is also borrowed as immutable
    *y += 1;
    println!("{}", x); // This will work after the mutable borrow is done
}
```
- In the above example you will get an error because you are trying to access `x` while it is borrowed as mutable by `y`. 
- You can only access `x` after the mutable borrow is done as mentioned below
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("{}", x); // This will work after the mutable borrow is done
}
```
- This will work because as expected because are not trying to access `x` while it is borrowed as mutable by `y`. You can only access `x` after the mutable borrow is done.

### Example for 2nd rule:
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &mut x; 
    *y += 1;
    *z += 1;
    println!("{}", x); // This will work after the mutable borrow is done
}
```

- This above example will give an error because you are trying to create two mutable borrows of `x` at the same time. You can only have one mutable borrow at a time.
- So if you wanted to browse z as mutable borrow of x you can do it like this:
after y is done you can create z as mutable borrow of x.

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    let z = &mut x;
    *z += 1;
    println!("{}", x); // This will work after the mutable
}
```

### Example  for 3rd rule:

```rust
fn main() {
    let x = 5;
    let y = &x;
    let z = &x; 
    println!("x: {} y: {}", x, y); // This will work after the mutable borrow is done
}
```
- In the above example you will not get an error because you are trying to create two immutable borrows of `x` at the same time. You can have multiple immutable borrows at the same time.

### Example for 4th rule:
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &x;
    *y += 1;
    println!("{}", x);
 } 
```
- This above example will give an error because you are trying to create a mutable borrow of `x` and an immutable borrow of `x` at the same time. You cannot have a mutable borrow and an immutable borrow at the same time.
