// Used to bring into scope
// Could use to str::io::stdin in a inline context
use std::io;
// Method that will be execuded first for any rust program
fn main() {
	// Macro for printing a line to concole. 	
	// Macros are denoted by !
	println!("Guess the number!");
	println!("Please input your guess.");
	// mut notes that this variable is mutable, by defualt all rust variables are imutable
	// String::new() assigns a new empty string
	let mut guess = String::new();
	// reades in the value from the console
	// &mut is used to point to the guess mutable variable 
	// read_line returns a Result enum, this result has type Ok or Err
	// if Err expect will cuase the program to crash
	// if Ok then the expect will use the value supplied by the user
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("You guessed: {guess}");    
}
