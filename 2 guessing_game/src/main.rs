extern crate rand;	// use crate imported in .toml dependencies

use std::io;	// Prelude: eliminates std:: from preceding io::stdin() later
use std::cmp::Ordering;
use rand::Rng;	// ensures Rng "trait" is in scope for a method to work

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1,101);

	println!("The secret number is: {}", secret_number);

	loop {
		println!("Please input your guess.");

		// Let = immutable, + mut = mutable variable
		// inverse type from String::new() so no need for explicit declaration
		let mut guess = String::new();

		// &mut guess = pass mutable reference; otherwise &guess refences immutable
		io::stdin().read_line(&mut guess)
		.expect("Failed to read line");	// read_line provides a Return value. If error, "panic!" occurs and expect is called ("crash on error")

		// this variable shadows the previous guess variable allowing us to reuse it and modify its type and convert it
		// trim() : removes whitespace before and after value including the '\n' resulting from pressing enter
		// parse() : parses the string into a number, but needs to know what type of number (could be i32 (signed), u32, i64, etc...)
		// match statment is used to handle errors. the Result enum is returned by parse(), but each contains more info
		//
		let guess: u32 = guess.trim().parse() {
			Ok(num)	=> num,		// Sets the unwrapped Ok value to num and "=> num" returns it
			Err(_)	=> continue,	// "_" allows us to catch all errors, continue makes loop start new iteration
		};

		println!("You guess: {}", guess);

		// Uses Ordering enum's with match which "takes a value of a type, and lets you 
		//	create an 'arm' for each possible value"
		// "match is really useful, and is used often in Rust"
		match guess.cmp(&secret_number) {
			Ordering::Less		=> println!("Too small!"),
			Ordering::Greater	=> println!("To big!"),
			Ordering::Equal		=> {
				println!("You win!");
				break;	// breaks the infinite loop
			}
		}
	}
}
