Rust provides three types of structs: classic C structs, tuple structs, and unit structs. Here’s an explanation and example of each:

1. Classic C Struct

A classic C struct is similar to structs in C or other languages. It has named fields, which makes it easy to access individual values.

Example:
```
  struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);
}
```

Key Points:
Fields are named and can be of different types.
Access fields using dot notation (person.name).
Useful for creating clear, structured data types.


2. Tuple Struct
   
A tuple struct is like a tuple but with a unique name for the struct. The fields are accessed by their positional index instead of names.

Example:
```
struct Point(i32, i32, i32);

fn main() {
    let origin = Point(0, 0, 0);

    println!("x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
}
  ```
Key Points:
Fields don’t have names; they're accessed by position (origin.0, origin.1).
Useful when the structure is simple or when naming fields is unnecessary.

///////////////////////////////////////////////////////////////////////////////////////////////////

3. Unit Struct

A unit struct is a struct with no fields. It acts as a marker or represents a singleton-like value.

Example:
```
struct Marker;

fn main() {
    let _marker = Marker; // Instance of a unit struct
}
```

Key Points:
Contains no data; only represents a type.
Often used for type-level information or as a zero-sized type (ZST) for traits or generics.
Comparison of Struct Types
Type	Fields	Access	Purpose
Classic C Struct	Named fields	struct.field_name	Structured, descriptive data.
Tuple Struct	Positional fields	struct.index	Simplified structure, similar to a tuple.
Unit Struct	No fields	No fields to access	Marker, type info, or ZST use cases.
Choosing Between Them
Use classic C structs when field names add clarity.
Use tuple structs when the data structure is simple and field names aren’t needed.
Use unit structs for markers, traits, or types without associated data.
