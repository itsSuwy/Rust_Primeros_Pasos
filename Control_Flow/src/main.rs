fn main() {
    let number: u8 = 8;
    let number_1: u8 = 9;
    // If expression
    if number>91{ // Just 1 condition
        println!("Condition is true!");
    } else {
        println!("Condition is false!");
    }
    if number_1 % 2 == 0 { // Multiple condition
        println!("Number is divisible by 2!");
    } else if number_1 % 3 == 0 {
        println!("Number is divisible by 3!");
    } else if number_1 % 5 == 0 {
        println!("Number is divisible by 5!");
    } else{
        println!("Number is not divisible by 2, 3 or 5!");
    }

    // Using if in a let statement
    let condition: bool = false;
    let number_1: u8 = if condition{5} else {6}; // The variable number_1 will be 5 if condition is true and be 6 if condition is false!
    // Note: Both elements must be of the same data type
    println!("{number_1}");
}
