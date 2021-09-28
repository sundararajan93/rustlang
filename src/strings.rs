/*
1. Primitive String =  Immutable, fixed-length  string somewhere in memory
2. String = Growable, Heap-allocated data structure - Use when you need to modify or own string data
*/

pub fn run(){
    println!("******* STRINGS ********");

    // Growable & Mutable String
    let mut hello = String::from("Hello ");

    // Get length of String
    println!("Length - {}", hello.len());

    // Pushing a char to string (we can push only one char at a time)
    hello.push('W');
    println!("{}", hello);

    // Pushing multiple string
    hello.push_str("orld!");
    println!("{}", hello);

    // Capacity in bytes
    let mut cap_string = String::with_capacity(15);
    
    cap_string.push_str("Hello Rust!");
    println!("capacity: {}", cap_string.capacity());

    // check if empty
    println!("Empty: {}", cap_string.is_empty());

    // Check if contains
    println!("Contains 'Rust' - {}", cap_string.contains("Rust"));


    // Replace 
    println!("Replace 'Rust' with 'Rustlang', {}", cap_string.replace("Rust", "Rustlang"));

    // Loop through string with Whitespace
    for word in cap_string.split_whitespace(){
        println!("{}", word)

    }
    
    // Assertion Testing
    assert_eq!(11, cap_string.len());
}