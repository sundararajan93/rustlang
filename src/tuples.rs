/*
Tuples are group together values of different data types
Max 12 elements
*/

pub fn run() {
    println!("******** TUPLES *********");

    // Creating a Tuple with two string value and one integer value
    let person: (&str, &str, i8) = ("John", "Cricket", 59);
    println!(
        "{name} like {sport} and {name} scored {score}",
        name = person.0,
        sport = person.1,
        score = person.2
    );

    // printing entire tuple
    println!("Tuple - {:?}", person);
}
