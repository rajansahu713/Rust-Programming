fn main() {

    //tuples
    println!("------------------Tuples-----------------");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    // accessing particular elements of a tuple
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // or accessing using indexing
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);


    // arrays
    println!("------------------Arrays-----------------");
    // creating a array
    let a = [1, 2, 3, 4, 5]; // array of 5 integers
    println!("The value of a is: {:?}", a);

    let b:[f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0]; // array of 5 floats
    println!("The value of c is: {:?}", b);

    let c = [3; 5]; // array of 5 integers, all initialized to 3

    println!("The value of c is: {:?}", c);

    // accessing particular elements of a array
    println!("The value of a[0] is: {}", a[0]);
    println!("The value of a[1] is: {}", a[1]);
    println!("The value of a[2] is: {}", a[2]);


}