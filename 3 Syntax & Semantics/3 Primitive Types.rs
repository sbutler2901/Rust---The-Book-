// Booleans
let x = true;
let y: bool = false;

// char - four bytes rather than one
let x = 'x';
let two_hearts = '❤️'

// Numeric types
// Consist of two parts: the category, and size. Ex. u16 = unsigned type with 16 bits of size

let x = 42; // defaults to type i32
let y = 1.0; // defaults to type f64

// Signed & unsigned, Fixed-size types
// valid bit sizes: 8, 16, 32, 64
let x: i8 = 5	// signed 8-bits
let y: u32 = 6	// unsigned 32-bits


// Variable-size types
let x: isize = 5	// variabled bit signed variable
let y: usize = 6	// variabled bit unsigned variable

// Arrays - immutable default
let a = [1, 2, 3];	// a: [i32; 3]
let mut m = [1, 2, 3];	// m: [i32; 3]

// array type = [T; N] - T = generic, N = compile time constant for length of array

let a = [0; 20]; // a: [i32; 20] all initialized to 0
a.len();	// returns array length
a[0];	// access first index

// Slices - reference ("view" into) another data structure. 
// Allow safe efficient accent to portion of an array w/o copying.
// Ex. reference only one line of a file read into memeory
// not created directly, but from exisiting variable binding
// of defined length, and can be mutable or immutable

// slice syntax:
// & - indicates slices are similar to references
// [] - within a range, let define length of the slice
// type: - &[T]

let a = [0, 1, 2, 3, 4];
let complete = &a[..];	// a slice containing all of the elements in a
let middle = &a[1..4];	// a slice of a: only the elements 1, 2, & 3


// Tuples - ordered list of fixed size

let x: (i32, &str) = (1, "hello");	// &str = "string slice"

let mut x = (1, 2);	//x: (i32, i32)
let y = (2,3); 	// y: (i32, i32)

x = y;

// destructuring let:

let (x, y, z) = (1, 2, 3);
println!("x is {}", x);

(0,);	// single element Tuple
(0);	// zero in parentheses

let tuple = (1, 2, 3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;


// functions

fn foo(x: i32) -> i32 { x }

let x: fn(i32) -> i32 = foo;




