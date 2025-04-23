## loops
- Loops are a fundamental concept in programming that allow you to execute a block of code repeatedly based on a condition. In Python, there are two primary types of loops: `for` loops and `while` loops. This section will cover the following topics:

- **For Loops**: Used to iterate over a range of values or elements in a collection.
- **While Loops**: Used to execute a block of code as long as a specified condition is true.
- **Loop Control Statements**: Used to alter the flow of a loop, including `break`, `continue`, and `return` statements.

1. iterate over a range of numbers using a for loop.
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("value {}", i);
    }
}
```

2. How to print value and index using for loop.
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for (index, value) in arr.iter().enumerate() {
        println!("index {} value {}", index, value);
    }
}
```

3. How to use while loop.
```rust
fn main() {
    let mut i = 0;
    while i < 5 {
        println!("value {}", i);
        i += 1;
    }
}
```
4. How to use loop control statements.
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        if *i == 3 {
            continue; // skip the value 3
        }
        println!("value {}", i);
    }
}
```
5. How to use break statement.
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        if *i == 3 {
            break; // exit the loop when value is 3
        }
        println!("value {}", i);
    }
}
```

6. Loop with label.
```rust
fn main() {
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2 {
                break 'outer; // exit the outer loop when i and j are both 2
            }
            println!("i: {}, j: {}", i, j);
        }
    }
}
```
7. loop with continue statement.
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        if *i == 3 {
            continue; // skip the value 3
        }
        println!("value {}", i);
    }
}
```

8. loop use as expression.
Break statement can be used as an expression to return a value from a loop.
```rust
fn main() {
    let mut i = 0;
    let result = loop {
        i += 1;
        if i == 5 {
            break i * 2; // exit the loop and return the value of i * 2
        }
    };
    println!("result {}", result);
}
```



