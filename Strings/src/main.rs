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

    // Life of a variable
    let x:u8 = 9;
    {
        let y:u8 =10 + x; // This value only lives in these brackets!
        println!("{y}");
        println!("{x}");
    } // It gets deleted here!
    println!("{x}"); // But x survives!
    //println!("{y}");
}
