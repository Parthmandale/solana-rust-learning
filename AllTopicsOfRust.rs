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

//////////////////////////////////////////////////////////////////////////////

/* STRUCT */

struct User {
    username: String,
    email: String,
    active: bool,
}

let user1 = User {
    username: String::from("parth"),
    email: String::from("parth@gmail.com"),
    active: true,
}

/*ENUM  */

enum Addr {
    // variants
    V4(u8, u8, u8, u8),
    V6(String),
    Unknown,
}

let home = Addr::V4(1,2,3,9);
let city = Addr::V6(String::from("::Pune"));
let unknown = Addr::Unknown;

///////////////////////////////////////////////////////////////////////////////

/* OWNERSHIP & MOVE SEMANTICS */

/* Each value in Rust has a variable that's called it's owner
Every value in rust ONLY have a single owner
when the owner goes out of scope, this value will be dropped.
*/ 

let s3 = String::from("block string");
{
    let s4 = s3; // s3 is moved into this block
    println!("Inside block: {}", s4);
} // here the scope finishes, so s4 is dropped here and the memory is freed

// println!("{}", s3); // Error! s3 is no longer valid | because in block the owner of s3 was changed to s4, but now as we are out of scope we can not access it.

///////////////////////////////////////////////////////////////////////////////

/* REFERENCES & BORROWING */

// & => means it is reference so it just used as view function, you can read but can not modify it.


fn calculate_length(s: &String) -> usize {
    s.len;
}


// this s1 has the ownership of the string
let s1 = String::from("hello");
// we calculate length but do not transfer ownership of value of the string to the new variable that is parameter of the function,
// we just tell it that here it is you can read it but you cannot modify it and s1 is still the owner
let len = calculate_length(&s1);


fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
// &mut - means refence and I';m sending hello in here but change function can update it just for its own function. here
// the owner is still s
change(&mut s);

///////////////////////////////////////////////////////////////////////////////

/* TRAITS */

// trait = interfaces in solidity, onlt function is declared and not defined.
// here in trait a function can also be already speacified. liek abstract
trait Summary {
    // &self - means we can access fields of structs and return string.
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}

// to implement the summary traits which specifies or defines the function called summararize. 
// if we have multiple functions declared inside summary trait then we will have to implement every function for the article Struct 
impl Summary for Artcile {
    fn summarize(&self) -> String {
        format!("{}...", &self.content[0..50]) // reading first 50 value of content string
    }
}



















