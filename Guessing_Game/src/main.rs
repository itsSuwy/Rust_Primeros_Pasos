use std::io; // We bring the input / output library from de standar (std) library
use rand::Rng; // Library to generate random numbers
use std::cmp::Ordering; // Library to compare inputs

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // our function to generate random number
    println!("The secret number is: {secret_number}");
    println!("The secret number is: {}", secret_number); // Two ways to print into the terminal
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // We create a mutable variable to store the user input that its bound ot a new, empty string
        io::stdin() // Here we are receiving the input from the keyboard of the user
            .read_line(&mut guess) // Stores whatever the user types into our variable called guess
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){ // Convert guess to a unsigner character of 32 bits, trim removes the spaces at the beggining and the end and parse try to convert the input into a number
            Ok(num)=>num, // In case parse could convert the string into a number
            Err(_)=> continue, // In case parse couldnt, it will be an infinite loop until the user types the correct number
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) { // Compare the guess and the secret number
            Ordering::Less => println!("Too small!"), // In case the guess is smaller
            Ordering::Greater => println!("Too big!"), // In case the guess is bigger
            Ordering::Equal =>{ // In case the guess is equal to the secret number
                println!("It matches with the secret number!\nYou win! :-)");
                break; // Just like C hahaha
            }
        }
    }
}
