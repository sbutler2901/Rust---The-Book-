// Vector - dynamic or 'growable' array - Vec<T> - T = any type allowed
// Always allocated on the heap
// vec! - macro similar to println!
// macros note - can use () or [], but conventuatlly () with println! and [] with vectors
// vectors must know the size of T at compile time - if this can be known then use Box type
let v = vec![1,2,3,4,5];	// v: Vec<i32>

let v = vec![0; 10]		// ten 0s

// us [] to access index of vector like arrays
let v = vec![1, 2, 3, 4, 5];
println!("The third element of v is {}", v[2]);

// must use usize for accessing an index of the vector
let i: usize = 0;
let j: i32 = 0;
// works
v[i];
// doesn’t
v[j];

// Accessing an index that doesn’t exist causes a panic
// use get or get_mut that return None when given an invalid index
let v = vec![1, 2, 3];
match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
}


// Iterating

// using for
for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}

// Cannot iterate more than once if taking ownership
let v = vec![1, 2, 3, 4, 5];

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
// won't work
for i in v {
    println!("Take ownership of the vector and its element {}", i);
}

// Can iterate multiple times if only reference it:
for i in &v {
    println!("This is a reference to {}", i);
}
// works
for i in &v {
    println!("This is a reference to {}", i);
}

// Add elements?
v.push(5);

