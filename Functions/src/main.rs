fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(-55,'D');
}
fn another_function(){
    println!("Hello!, from another function");
}

fn print_labeled_measurement(value: i32, unit_label:char){ // In this case value has to be a integer of 32bits and unit_label a char
    println!("The value of parameter 1 is {value} and parameter 2 is {unit_label}");
}
// Note that we have to express explicitly the type of data that the function receives as parameter