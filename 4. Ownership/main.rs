fn main() {
    println!("--------------- Ownership Intergers ---------------");
    let a = 5;
    let b = a; // Copying the value of `a` into `b`
    println!("a: {}, b: {}", a, b); // a is still valid because i32 implements the Copy trait

    println!("{}", a); // 4 bytes

    println!("--------------- Ownership Strings ---------------");
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2
    // println!("{}", s1); // s1 is no longer valid because it was moved into s2    
    println!("{}", s2); // s2 is valid because it was created from s1

}
