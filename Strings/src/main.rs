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

    // How to copy a string inside another variable
    let x: u8 = 9; // This perfectly works!
    let y: u8 = x;
    println!("The value of x is {x}");
    println!("The value of y is {y}");

    // But what about this?
    let str = String::from("Hello, world!");
    let stre_1 = str;
    //println!("{str}"); // This line will crash our program!
    println!("{stre_1}");
    // When you have one string and you store it inside another string in that way, rust will liberate the memory from the original string
    // To copy and have two identical strings, we have to do this
    let s1 = String::from("Hello, world!");
    let s2 = s1.clone(); // Copy but dont destroy the original variable
    println!("{s1} from S1");
    println!("{s2} from S2");

    // Why does it work with integers but NOT with strings?
    // Rule: For 'let y = x' to leave BOTH variables alive, the type must implement the COPY trait.
    // This only applies to types with a FIXED and KNOWN size in bits at compile-time.
    // STACK: Where types with fixed sizes live (Integers, Unsigned, Floats, Bool, Char).
    // They are cheap to duplicate, so Rust performs a quick bitwise copy in the Stack.
    // HEAP: Where dynamic, unknown-size data lives (Strings).
    // Even though the String pointer is in the Stack, the actual text is in the Heap.
    // Rust FORBIDS 'Copy' here to prevent pointers from duplicating and messing up hardware memory (Double Free Error).


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
