fn main() {
    //There are three types of loops
    // For Loops, While loops and Loops
    // - Loop: It executes a block of code over and over until you explicitly tell it top stop
    // Example:
    /*loop {
        println!("Again!");
        // This code will be executing until we stopped manually the program
    }
    */
    // Fortunately we can stop infinite loops using a the keword "Break" to stop it
    // Also we can use the keyword "Continue" to tell the program to skip the next part of the code and return to the start of the loop
    // For example
    let mut counter: u8 = 0; // The variable that will indicate when to stop
    let result = loop { // Statement and Expression to store the value
        counter += 1; // In every iteration of the loop our counter will increment
        if counter == 10 { // When the counter is equal to ten, the if its true!
            break counter *2; // We multiply with 2 our counter (we will have a 20!)
        }
    };
    println!("The result is {result}");
    println!("The counter is {counter}");

    // Loop anidate
    // Did you know that you can have 2 loops anidates and bautize them?
        let mut count = 0; // We create a mutable variable to control the loop
        'counting_up: loop { // We bautize our loop as 'counting up!
            println!("count = {count}"); // We print the value of out variable count
            let mut remaining = 10; // Our second variable
            loop { // This is our second loop
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up; // Here we stop our main loop
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
        // Basically we start with two important variables: Count and Remaining
        // And two loops: counting_up and our generic loop
        // So, remaining starts with a value of 10, and then gives that value to the second loop
        // Both conditionals evaluate if remaining has a value of 9, but in our first iteration it doesn't have that value
        // And the second conditional its false at that moment, so we take 1 from our original value (ten)
        // And we restart the loginc but now the first conditional matches! So we break that "generic" loop
        // So we return to the main loop and now we add 1 to our count, and we repeat the proces one more time
        // So now our variable count has a value of 2, and it matches with our second conditional
        // In our third iteration, now the second loop with the second conditional matches!
        // It calls a function "break 'counting_up;", <- when it has a ' before the name, it refers to a bautized loop!
        // At the end, it breaks our main loop, and now we return to the next part of the code finishing both loops!
    // While loops
    let mut number: u8 = 3;
    while number != 0 { // The expression that always has to be true, when its false it's stops
        println!("{number}");
        number -= 1;
    }
    println!("End number is {number}");

    // For loops
    let a: [u8;5] = [10, 20, 30, 40, 50];
    for element in a { // Rust knows the size of the array, we dont have to specify it
        println!("the value is: {element}");
    }
    for number in (1..4).rev() { // This for goes from 3 til 1, excluding de 4 (.rev() makes the reverse posible)
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}