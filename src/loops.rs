// Loops - used to iterate until a specific condition is met

pub fn run() {

    println!("********* LOOPS **********");
    // let mut count = 1;

    // Infinite Loop

    // loop {
    //     count += 1;
    //     println!("Count - {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop

    // while count <= 100 {
    //     //println!("Count - {}", count);

    //     if count % 7 == 0{
    //         println!("FizzBuzz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else if count % 2 == 0 {
    //         println!("Fizz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     count += 1;
    // }


    // for Loop

    for number in 1..100{
        if number % 7 == 0{
            println!("FizzBuzz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else if number % 2 == 0 {
            println!("Fizz");
        } else {
            println!("{}", number);
        }
    }

}
