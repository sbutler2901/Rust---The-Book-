fn main() {
    print_sum(5, 6);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

// this will not work : explicit type declaration required
fn print_sum(x, y) {
    println!("sum is: {}", x + y);
}

// Rust only returns 1 value
// last line of function determins what it returns
fn add_one(x: i32) -> i32 {
    x + 1	// a semicolon would break this, 
}

// Expressions - return a value

// Statements - do not return a value
// Declaration statments
x = y = 5	// not allowed

let mut y = 5;	// 
let x = (let y = 6); 	// not allowed, an assignment's value is () not the value, i.e., x has value '()', not 6
// this is why the previous function could not have a semicolon for the return value as that would return '()' ie the result of a Statement


// Early returns:
fn foo(x: i32) -> i32 {
	return x;	// dont use 'return' at end of function (poor style)
	// never ran
	x+1
}

// Diverging functions - functions that do not return
fn diverges() -> ! {
	// this causes a crash (therefore not returning) and so has type '!', which is read 'diverges'
	panic!("This function never returns!");
}
// diverging function can be used as any type:
let x: i32 = diverges();
let x: String = diverges();

// Function pointers
// f is a variable binding pointing to a function taking i32 as argument and retuning i32
let f: fn(i32) -> i32;

fn plus_one(i: i32) -> i32 {
	i + 1
}

// w/o typ inference
let f: fn(i32) -> i32 = plus_one;

// w/ type inference
let f = plus_one;

let six = f(5);
