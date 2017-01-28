fn main() {
    //Patterns
    let (x, y) = (1, 2);

    //Type annotations
    //	i = signed, u = unsigned
    let x: i32 = 5;
    let y = 6	// type infered

    // Mutable
    let mut z = 7;	//type inferred for this mutable variable

    // Initializing bindings
    let a: i32;	// warning will occur, but because not used will still run

	// String interpolation (CSCI term meaning "stick in middle of a string")
    println!("The value of x is: {}", x);

    // Scope
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); // This won't work

    // Shadowing
    let x: i32 = 8;
	{
	    println!("{}", x); // Prints "8"
	    let x = 12;
	    println!("{}", x); // Prints "12"
	}
	println!("{}", x); // Prints "8"
	let x =  42;
	println!("{}", x); // Prints "42"

	let mut x: i32 = 1;
	x = 7;
	let x = x; // x is now immutable and is bound to 7

	let y = 4;
	let y = "I can also be bound to text!"; // y is now of a different type
}
