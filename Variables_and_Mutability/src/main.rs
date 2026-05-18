
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
    println!("The value of float is: {float}");
    let float_2: f32 = 3.5; // Here, we are explicitly telling rust: This variable must be of 32 bits
    println!("The value of float is: {float_2}");
    // In Rust the floating-points follow the IEEE-754

    // Numeric Operations
    // Rust supports the basic mathematical operations

    // Addition:
    let sum: u32 = 5 + 10; // We use u because they are two positives
    println!("The value of sum_1 is: {sum}");
    let sum_2: i32 = 5 + 10; // This work too, but its unnecesary to use i because the result will be positive
    println!("The value of sum_2 is: {sum_2}");
    let sum_3: f32 = 10.5 + 20.4; // This can work!
    println!("The value of sum_3 is: {sum_3}");
    //let sum_2: u32 = 5 -10; // THIS IS INCORRECT, NEVER DO THIS WITH THE U
    // Remember that u must store a positive value, if we give a negative, the program will generate a panic error

    // Substraction:
    let diff: i32 = 5 - 10; // This is correct, the result will be negative, and the i can store it
    println!("The value of diff_1 is: {diff}");
    let diff_2: f32 = 95.5 - 10.9; // This can work!
    println!("The value of diff_2 is: {diff_2}");
    //let diff_2: f32 = 95.5 - 10; // This wonet div_2: f32 = 5 / 10;t work! You cant substract a integer from a floating-point!

    //Multiplication:
    let mult_1: u32 = 5 * 10; // This works!
    println!("The value of mult_1 is: {mult_1}");
    let mult_2: i32 = 5 * -10; // This works too!
    println!("The value of mult_2 is: {mult_2}");
    let mult_3: f32 = 4.3 * 2.9; // This also works!
    println!("The value of mult_3 is: {mult_3}");
    // let mult_4: u32 = 5 * 10.9 // This cant work!

    // Division:
    let div_1: u32 = 5 / 10; // This works but there is a Fine print!
    // The result will be an integer too (in this case positive), i wont have ani decimal point
    println!("The value of div_1 is: {div_1}");
    //let div_2: f32 = 5 / 10; // This doesnt work because Rust is reading two integers instead of floating-point
    // There are two ways to correct this!
    // Option 1:
    let div_3: f32 = 5.0 / 10.0; // We add the decimal point to the number
    println!("The value of div_3 is: {div_3}");
    // Option 2:
    let div_3: f32 = 5f32 / 10f32; // We add the format to the numbers we want to divide
    println!("The value of div_3 is: {div_3}");
    // Both ways can perfectly work!
    // But there is a problem....
    // What about if we try to do this?
    let a: u32 = 9;
    let b: u32 = 10;
    //let div_4: f32 = af32 / bf32; // Rust doesnt know the the heck is af32 or bf32!
    // And the program will crash :(
    // Instead of this, you can do this
    let div_4: f32 = (a as f32) / (b as f32); // This works!
    println!("The value of div_4 is: {div_4}");
    // But there is a problem, a great power comes a great responsability
    // Using as could crash your code if you dont know how to use it
    // For example if you have a variable that is size is 8 bits, and you give
    // (variable as u64), there wont be a problem, because the size if perfect for your variable
    // But if your variable is u64 and you want to use (variable as f32)
    // The size of bits is smaller than the original variable! BOOOOM! Information lost and you'll get an strange number!
    // So, always use as with caution :)

    // Remainder
    let rem_1: u32 = 43 % 3;
    println!("The remainder of rem_1 is: {rem_1}");

    // And thats all of basic math operations!

    //let guess: u32 = "42".parse().expect("Not a number!");

    // The boolean type of data!
    // A boolean can have two different values: True o False, also it consumes 4 bytes
    let bool_1 = true; // Implicit way to declare it <- Rust has to figure out the type of data
    println!("The value of bool_1 is: {bool_1}");
    let bool_2: bool = true; // Explicit wat to declare it <- You tell rust the especific type of data
    println!("The value of bool_2 is: {bool_2}");

    // The character type of data!
    // Represents just 1 character, can be a number, letter, digit or emoji!
    // Also it consumes 4 bytes
    let char_1: char = 'a'; // Note that we uses single quotation mark (' ') to denote it, a string uses doble quotation mark ("")
    println!("The value of char_1 is: {char_1}"); // Note that we used the explicit way
    let char_2 = 'b'; // Here is the implicit form to express it
    println!("The value of char_2 is: {char_2}");

    // With that, we are already covered all the scalar data types!
    // Now it's time to talk about the "Compount" types!
    // The Tuple
    // The tuple is a way to agroup together a number of values with a varity of types into one compound type
    // However, The tuple cant grow or shrik once declared
    // Example:
    let tup: (i32, f64, u8) = (-400,3.5,9); // Here we declarate and filled the tuple
    // Another way to do it
    let tupl = (500,5.6,4); // First we fill our tuple
    let (x, y, z) = tupl; // Then we assign each valor to a variable in a implicit way
    // This is known as destructuring
    println!("The values of x, y and z are: {x} {y} {z}");
    // Another way to do it is like this:
    let tuple: (i32, f64, u8) = (-10,2.8,2); // The explicit way to tell Rust the size of the elements
    let (x,y,z) = tuple;
    println!("The values of x, y and z are: {x} {y} {z}");
    // To access to a tuple element, we use a period (.)
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // To access to the first element
    let six_point_four = x.1; // To access to the second element
    let one = x.2; // To access to the third element

    // The array type!
    // Unlike a tuple, every element of an array must be of the same type
    // You cant have a char and an integer, or an unsigned integer with a signed integer
    // Example
    let a = [1, 2, 3, 4, 5]; // The implicit way to define an array
    let b: [u8;5] = [1, 2, 3, 4, 5]; // The explicit way to define an array (We decide the size of each elementes to store)
    // Notice how we use the same kind of data inside the array
    // Another way to use arrays
    let c = [3;5]; // Implicit declaration
    // Its the same as writing:
    let c_1 = [3,3,3,3,3]; // Wow!
    // Btw, the explicit sintaxis is like this:
    let c_2:[u8;5] = [3;5]; // Explicit way
    let c_3:[u8;5] = [3,3,3,3,3]; // Explicit way
    // Both works in the same way!

    // How to access to an array element?
    let my_array:[u8;3] = [3,3,3];
    let first = my_array[0];
    let second = my_array[1];
    let third = my_array[2];
    // Thats how we access to each element of the array!

}
