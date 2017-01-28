// Infinite loop - until some terminating statement is reached
loop {
	println!("Loop forever!");
}

let mut x = 5;	// mut x: i32
let mut done = false;	//mut done: bool

// While
while !done {
	x += x-3;

	println!("{}", x);

	if x % 5 == 0 {
		done = true;
	}
}

while true {}	// this is possible, but it is much better to use 'loop'

// for

// C-style
for (x = 0; x < 10; x++) {
	printf("%d\n", x);
}

// Rust
// Upper bound is exclusive = doesn't include last number = 0-9
for x in 0..10 {
	println!("{}", x);	// x: i32
}

// Rust Abstractly
// 'var' = value of each loop is bound to this
// expression = an item that can be converted into an iterator using "IntoIterator"
for var in expression {
	code
}

// Enumerate

// On ranges - .enumerate() - keeps tracks of number of times looped
for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
}
// output:
index = 0 and value = 5
index = 1 and value = 6
index = 2 and value = 7
index = 3 and value = 8
index = 4 and value = 9

// On iterators:
let lines = "hello\nworld".lines();

for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}
// output:
0: hello
1: world


// Ending iteration early
// break - breaks out of the loop
// continue - skips to the next iteration
// return - explicit return breaks loop

// improved looping compared to prior
// loops forever until condition is satified
let mut x = 5;

loop {
	x += x - 3;

	println!("{}", x);

	if x % 5 == 0 { break; }
}

for x in 0..10 {
	if x % 2 == 0 { continue; }

	println!("{}", x);
}

// Loop label
// 'outer & 'inner are labels used to specify which loop to apply continue/break statements to
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        if x % 3 == 0 { break 'outer; }
        println!("x: {}, y: {}", x, y);
    }
}

