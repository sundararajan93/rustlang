// Structs - used to create a custom data types

// Traditional struct
struct Rgb{
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct NewColor(u8, u8, u8);

// Implementing Struct with a function
// creating struct
struct Person{
    first_name: String,
    last_name: String
}

// Implementing in function
impl Person {
    // Constructor function
    fn new(first:&str, last:&str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //function to get fullname
    fn get_fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
        
    }
    
    //function to change last name
    fn change_lastname(&mut self, last: &str){
        self.last_name = last.to_string();
    }
}

pub fn run(){

    println!("\n******* STRUCT *******");

    // creating variable using custom datatype Rgb
    let mut c = Rgb {
        red: 200,
        green: 120,
        blue: 26
    };

    println!("RED - {} BLUE - {} GREEN - {}", c.red, c.green, c.blue);

    // modifying the value
    c.blue = 150;

    second_run();

    let mut p = Person::new("Sundara", "Rajan");
    println!("FIRSTNAME - {}\nLASTNAME - {}", p.first_name, p.last_name);
    
    println!("{}", p.get_fullname());

    p.change_lastname("Raman");
    println!("{}", p.get_fullname());
}

// Creating Tuple variable

fn second_run(){
    let mut c = NewColor(255,0,120);
    println!("Tuple RGB - {:?}", (c.0, c.1, c.2));

    c.0 = 220;
    println!("Tuple RGB - {:?}", (c.0, c.1, c.2));

}