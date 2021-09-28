// Arrays are fixed length of elements with same datatype grouped together

use std::mem;

pub fn run() {
    println!("********* ARRAY **********");
    // creating an array
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Printing the array
    println!("{:?}", numbers);

    // Modifying the array and printing single value
    numbers[0] = 100;
    println!("First element modified as {}", numbers[0]);

    // Get array length
    println!("Length of the array - {}", numbers.len());

    // Memory occupied for an array
    //println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Calling after importing std library
    println!("Memory: {} bytes", mem::size_of_val(&numbers));

    // Slicing the array
    let slice: &[i32] = &numbers[0..3];
    println!("Sliced array - {:?}", slice);
}
