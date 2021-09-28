// Vectors are resizable arrays

use std::mem;

pub fn run() {
    println!("********* VECTORS **********");
    // creating a Vector
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Printing the Vector
    println!("{:?}", numbers);

    // Modifying the Vector and printing single value
    numbers[0] = 100;
    println!("First element modified as {}", numbers[0]);

    // Get Vector length
    println!("Length of the Vector - {}", numbers.len());

    // Memory occupied for an array
    //println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Calling after importing std library
    println!("Memory: {} bytes", mem::size_of_val(&numbers));

    // Slicing the Vector
    let slice: &[i32] = &numbers[0..3];
    println!("Sliced Vector - {:?}", slice);

    // Adding values to vector
    numbers.push(11);
    numbers.push(12);
    println!("Vector after modification - {:?}", numbers);

    // Pop the last added value
    numbers.pop();
    println!("Vector after modification - {:?}", numbers);

    // Looping through Vector 

    for number in numbers.iter(){
        println!("Number: {}", number);
    }

    for n in numbers.iter_mut(){
        *n *= 2;
    }

    println!("Vector multiplied by 2 {:?}", numbers);
}
