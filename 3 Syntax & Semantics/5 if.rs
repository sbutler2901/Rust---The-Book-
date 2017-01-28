let x = 5;

// normal control flow
if x == 5 {
	println!("x is five");
} else if x == 6 {
	println!("x is six");
} else {
	println!("x is not five or six :(");
}


// Dynamic assignment of values
let y = if x == 5 {
	10
} else {
	15
};	// y: i32

let y = if x ==5 { 10 } else { 15 };	// y: i32