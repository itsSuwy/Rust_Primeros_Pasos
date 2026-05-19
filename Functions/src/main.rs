fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(-55,'D');
    another_function2();
    another_function3();
    let value_1 = return_value();
    println!("Returning a value with the Rust way = {value_1}");
    let value_2 = return_value2();
    println!("Returning a value with the explicit way = {value_2}")
}
fn another_function(){
    println!("Hello!, from another function");
}

fn print_labeled_measurement(value: i32, unit_label:char){ // In this case value has to be a integer of 32bits and unit_label a char
    println!("The value of parameter 1 is {value} and parameter 2 is {unit_label}");
}
// Note that we have to express explicitly the type of data that the function receives as parameter
// Two kind of functions: Statements and Expressions
// - Statementes: Instructions that doesn't return a value
// - Expressions: Evaluate to a resultant value
// For example: let x: u8 = 7 <- This is a statement, it creates and assigns a value to a variable, but doesn't return the value

fn another_function2() {
    let y: u8 = // < Our statement
        { // <- Start of the expression
        let x = 3; // After the execution of this {}, x dies :(
        x + 1
    }; // <- End of the expression
    println!("The value of y is {y}");
} // WHY DOES THIS WORK?!
// The let function is a statement BUUUUT, the {} are an expression that indicates to the statement to store the value that it returns!

// There are two ways to define a function: If returns a value or not
// We already saw the body of a function that doesnt raturn a valur to the main
// Just writes the name of the function, some () or (type and name of the variables here)

// Non-return value function
fn another_function3(){ // Sometimes the () can be empty, sometimes they can receive a parameter
    println!("Im just a non return value funciont!");
}

// Return value funcion
fn return_value() -> u8{ // First we have to express the type of data that we are returning with this function (in this case it's an unsidgned-8bits)
    // Then we have to express the value we want to return, there are two ways
    // For example
    let x: u8 = 10;
    let y: u8 = 11;
    // In this example we are making two variables that we want to sum
    x + y // Here we are returning the value of the sum of x and y
    // That was known as the rust way to return a value, but also we can return it with an explicit way
}

fn return_value2() -> u8{
    let x: u8 = 10;
    let y: u8 = 11;
    return x + y // Explicit way to return a value
    // Both ways work very well when the main topic its to return a value!
}

// A pro tip: If you see a ; next to the line, it means it's a statement (doesn't return a value)
// But if you dont see it, it means it's an expression!