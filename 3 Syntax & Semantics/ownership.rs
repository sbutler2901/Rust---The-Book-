// Ownership

// Variable binds 'have ownership' of what they're bound to
// when binding goes out of scope, rust will free the bound resources
fn foo() {
	// when v come into scope a new vector is created on the stack & allocates space on heap for its elements
	// when v goes out of scope at end of foo(), rust will clean up everything related to the vector, even the heap-allocated memory
	let v = vec![1,2,3];
}

// rust enures exactly one binding to any given resouce
let v = vec![1, 2, 3];
let v2 = v;

println!("v[0] is: {}", v[0]);	// This will produce error "...moved value..."

fn take(v: Vec<i32>) {
    // what happens here isn’t important.
}

let v = vec![1, 2, 3];
take(v);
println!("v[0] is: {}", v[0]);	// this will produce 'moved error' too because ownership of v has been taken from the funtion

// The details

// Rust allocates memory for an integer i32 on the stack, copies the bit pattern representing the value of 10 to the allocated 
// memory and binds the variable name x to this memory region for future reference.
let x = 10;

// v is memory on the stack containg a pointer to the vector data [1,2,3] stored on the heap
let v = vec![1, 2, 3];
// v2 is now a duplicate pointer to the heap vector object a 'shallow' stack copy
let mut v2 = v;
// v is invalidated afterwards to prevent data races:
v2.truncate(2);	// truncates the vector to just two elements through v2
// v still thinks there are three elements in the vector unaware of the changes v2 made to it
// this could cause a segmentation fault of allow an unauthorized user to read from memory they do not have access to
// thefore Rust forbids using 'v' after giving the ownership of the vector to 'v2'

// Copy types - These types add extra behavior allowing copying of data since they do not contain pointers to data somewhere else, copying is a full copy
let v = 1;	// v is an i32 which implements the copy trait = when we assign v to v2, a copy of the data is made
// unlike a move, though, we can still use v afterwards - i32 has no pointers to data elsewhere
let v2 = v;
println!("v is : {}", v);
// all primitive types implement the Copy trait
