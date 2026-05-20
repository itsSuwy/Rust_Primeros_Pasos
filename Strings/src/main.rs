use std::io; // Our input / output library
fn main() {
    // How to work with a string
    // There are two ways to work with a string
    let s = "Hola"; // The common way
    println!("{s}");
    //let mut str = String::new(); // It initializes a 24-byte structure in the Stack with 0 capacity, preparing the terrain to allocate memory in the Heap later.
    let mut s_1 = String::from("Hola 2"); // Creates dynamic memory to store our information
    println!("{s_1}");
    s_1 = String::from("Adios!"); // We can modify our variable (This will delete the previous information!)
    println!("{s_1}");

    // How to get an input from the user!
    println!("Input some text!"); // Step 1: The message hahaha
    let mut input = String::new(); // Step 2: We prepare the variable of string type (Remember new allocates memory but doesnt allocate memory in the heap )
        io::stdin() // We allor the code to read the keyboard
            .read_line(&mut input)// Step 3: Here we read te input from the user
            .expect("Failed to read line"); // Step 4: If something goes wrong, this line saves the code from a crash

    // Bonus: Life of a variable
    let x:u8 = 9;
    {
        let y:u8 =10 + x; // This value only lives in these brackets!
        println!("{y}");
        println!("{x}");
    } // It gets deleted here!
    println!("{x}"); // But x survives!
    //println!("{y}");
}
