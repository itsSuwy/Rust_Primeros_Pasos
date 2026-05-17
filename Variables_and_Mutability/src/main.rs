
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // This is a constant
// It will be in any part of the code, first we need to write the word "const"
// Then we write the name, after that the type and finally the value it'll store

fn main() {
    // Variables and mutability!
    let x = 6; // X CANT change its value
    let mut y = 8; // Y can change its value
    // This is known as inmutability, for example:
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    // Now if we try to change the value of y
    y = 9;
    println!("The value of y is: {y}"); // It changes the value of y !
    // but if we try to change the value of x
    //x = 10; // This line will provocate an error!
    //println!("The value of x is: {x}");

    // Time to shadow a variable!
    let k = 10;
    println!("The value of k is: {k}");
    let k = k +1;
    println!("The new value of k is: {k}");
    {
        let k = k * 2;
        println!("The new value of k is: {k}!");
    }
    println!("The final value of k is: {k}");

    // Differences between shadowing and mutability
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");
    // With shadowing, we can reuse the variable to store different types of data

    //let mut spaces_v2 = "      ";
    //spaces_v2 = spaces_v2.len();
    // With mutability, the new value has to be always of the same type of the original variable

    // Data types!
    // There are two types of data in Rust: Scalar and compound
    // In one hand: The Scalars!
    // They represent a single value, such as: Integers, floating-point numbers, Booleans and characters!.
    // Integer: Number without a fractional component
    // There are two types of integers: Unsigned and Signed
        // Unsiged doesnt have a sign (They are always positive!)
        // But signed integers has a sign, the can be positive and negative! Crazy, right?
    // To declare an integer we first have to express the lenght of the variable and the kind of sign
    // Length: 8-bit, 16-bit, 32-bit, 64-bit, 128-bit, architecture-dependent <- This one depends of the hardware
    // Signed: i8, i16, i32, i64, i128 and isize
    // Unsigned: u8, u16, u32, u64, u128 and usize
    // usize and isize will automatically assign the length of the variable depending on the hardware architecture

    // Floating-Point Types:
    // Basically, they are number with decimal points
    // There are just two types of length to this type, and they're sintaxis is the same
    // For 32-bit = f32
    // For 64-bit = f64 <- This is the standard when Rust is going to automatically assign the type
    // Example:
    let float = 2.5; // Rust assigns a f64 (64 bits)
    let float_2: f32 = 3.5; // Here, we are explicitly telling rust: This variable must be of 32 bits
    // In Rust the floating-points follow the IEEE-754

    let guess: u32 = "42".parse().expect("Not a number!");
}
