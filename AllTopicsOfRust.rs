/* VARIABLE AND MUTAILBILITY */

let x = 5; // immutable | by default signed integer 32 -> i32
let mut y = 10; // mutable
y = 15; // changed 

const MAX: u32 = 100_000;

//////////////////////////////////////////////////////////////////////////////

/* DATA TYPES AND STRINGS */

let a: i32 = 42;
let b: f64 = 3.14; //floating int
let c: bool = true;
let d: char = 'R';

let tup: (i32, f64, u8) = (500, 6.4, 1); // group of multiple variable, whereas array is just group of same type of variable
let arr: [i32; 3] = [1,2,3];

let mut s = String::from("hello");
s.push_str(", world");


/*Here, the & operator creates a string slice (&str) referencing a portion of the string s.
s[0..5] specifies a range that includes characters at indices 0 through 4 (start is inclusive, end is exclusive). This extracts "hello" as a view into the original string.
&s[0..5] creates a reference to this slice instead of copying the data.*/
/*In Rust, the & operator is used to create references, which are pointers to a value without taking ownership of it.*/

let slice: &str = &s[0..5]; 

//////////////////////////////////////////////////////////////////////////////

/* CONTROL FLOW */

let x = 10;
if x > 5 {
    println!("x is greater than 5");
}

else {
    println!("x is lesser than 5");
}

let y = if x > 5 { 10 } else {0};

//////////////////////////////////////////////////////////////////////////////

/* LOOPS */

// will stop only when break will appear
loop {
    println!("Infinite loop");
    break; // Stops the loop
}

// while loop

let mut n = 3;
while n > 0 {
    println!("{}", n);
    n -= 1;
}

// for loop

for i in 1..4 {
    println!("{}", i);
}


// match statements - just like switch statements

let num = 3;

match num {
    1 => println("One"),
    2 => println("two"), 
    3 => println("three"), 
    _ => println("Other"),  // if not none of the above
}

/* FUNCTION */

fn add(a: i32, b:i32) -> i32 {
    return a+b; // explicitly telling return with then apply ;
}
fn square(x: i32) -> i32 {
    x * x // no semicolon in returing this value
}

fn main() {
let addition = add(5,3);
let sq = square(5);

println!("addition: {}", addition);
println!("sq: {}", sq);
}






















