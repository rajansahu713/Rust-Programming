fn main() {
    // interger types

    let a: i32 = 6; // 4 bytes
    let b: i64 = 20; // 8 bytes

    let c: u32 = 68719; // unsigned 4 bytes
    let d: u64 = 40; // unsigned 8 bytes
    
    println!("{} {} {} {}\n", a, b, c, d);

    // floating point types
    let e: f32 = 3.14; // 4 bytes
    let f: f64 = 3.2; // 8 bytes
    
    println!("{} {}", e, f);

    // boolean types
    let g: bool = true;
    let h: bool = false;

    println!("{} {}", g, h);

    // character types
    let i: char = 'a';
    let k: char = '1';

    println!("{} {}", i, k);

}