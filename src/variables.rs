// Variables are references to the data
// Variables are immutable (cannot be reassigned)
// Rust is a block scoped language


pub fn run(){

    println!("******* VARIABLES *******");
    // Assigning a variable and value
    let name = "Justin";
    let age = 52;

    println!("{} is {}", name, age);

    // mutable variable
    let mut humidity = 65;
    println!("Humidity {}", humidity);

    humidity = 67;
    println!("Humidity changed to {}", humidity);

    // Assigning multiple variables

    let (my_name, my_age, my_city) = ("Karthik", 29, "Chennai");

    println!("Name: {}, Age: {}, City: {}", my_name, my_age, my_city);

}