// functions - Block of code for re-use (DRY - Don't Repeat Yourself)

//Main run function
pub fn run(){
    println!("********* FUNCTIONS **********");
    greet("Hello", "Sundar");

    let sum = add(5, 9);
    println!("Sum : {}", sum);


    // closure funtion - anonymous functions & can capture values from the scope in which they’re defined
    // let <variable_name> = | argument1: <type>, argument2: <type> | <expression>

    let prod_numbers = | number1: i32, number2: i32 | number1 * number2;
    println!("Product is {}", prod_numbers(3, 4));

}

// simple string argument function
fn greet(greeting: &str, name: &str){
    println!("{} {}, Nice to see you!", greeting, name);
}

// return value
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

