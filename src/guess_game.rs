use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn run(){
    println!("\n******* GUESS GAME *******");
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..10);

    println!("The secret number is {}", secret_number);
    
    loop {
        println!("\nGuess the number: ");
        let mut guess:String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let mut guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,   
        };

        let sad_face = '\u{1F614}';
        let angry_face = '\u{1F621}';
        let happy_face = '\u{1F601}';

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Low! {}", sad_face),
            Ordering::Greater => println!("Too High! {}", angry_face),
            Ordering::Equal => {
                println!("You got it!!! {}", happy_face);
                break;
            }
        }
    }        

}