pub fn run(){

    println!("******* FORMATTING CONSOLE OUTPUT *******");

    // To Print something from this file
    println!("This is printing from print.rs file");

    // To print an integer with with placeholder
    println!("Number: {}", 1);

    // Positional Arguments
    println!("{0} is {1} and {0} wants to {2}", "Brad", "sad", "sleep");

    // Named Arguments
    println!("{name} is {activity}", name = "John", activity = "playing");

    // Placeholder traits
    println!("Binary : {:b} Octal: {:o} HexaDecimal: {:x}", 10, 10, 10);

    // Placeholder debug traits
    println!("{:?}", (12, true, "Robert"));

    // Basic math
    println!("Addition {}", 10 + 5);
    println!("Subtraction {}", 10 - 5);
    println!("Multiplication {}", 10 * 5);
    println!("Division {}", 10 / 5);
    
}