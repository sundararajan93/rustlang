/*
Primitive Datatypes --- 
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits take into memory)
Floats  : f32, f64
Boolean: (bool)
Character: (char)
Tuples
Arrays
*/

pub fn run(){
    println!("******* DATA TYPES ********");

    // Integers by default take i32
    let int_number = 9;
    println!("Integer Number: {}", int_number);

    // Float by default take f64
    let float_number = 2.85;
    println!("Float Number: {}", float_number);

    // Adding the datatype explicitly
    let int64_number: i64 = 5454645533;
    println!("Int 64 Number: {}", int64_number);

    // Maximum numbers in datatypes
    println!("Max of i32: {}", std::i32::MAX);
    println!("Max of i64: {}", std::i64::MAX);
    println!("Max of i128: {}", std::i128::MAX);

    // Boolean
        let is_active = true;

        // can also be declared explicitly like the next line 
        // let is_active: bool = true;

        // Assigning boolean based on condition
        let is_greater = 10 > 5;

        println!("Active Status: {}", is_active);
        println!("Is Greater: {}", is_greater);   

    // Character

    let a = 'a';

    let smiley = '\u{1F60D}';

    println!("{} is {}", a, smiley);

}