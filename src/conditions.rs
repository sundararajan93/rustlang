//  Conditionals - Checks for conditions and act 
/*
&& - AND
|| - OR
> - Greater Than
< - Less than
<= - Less than or equals to
>= - Greater than or equal to
== - Equals to
!= - Not Equal to
*/

pub fn run(){
    println!("******** CONDITONS *********");
    // creating variables
    let age: u8 = 21;
    let has_id: bool = false;

    if age >= 21 && has_id{
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && has_id {
        println!("Bartender: Sorry, You have no access");
    } else {
        println!("Bartender: I'll have to see your ID.");
    }

    //Shorthand If/Else
    let is_adult = if age >= 21 { true } else { false };
    println!("Is He an Adult? {}", is_adult);
}

