
enum Movement{
    Up,
    Down,
    Left,
    Right,
}

fn move_ball(m: Movement){
    // Perform action depending on the Info (movement)
    match m{
        Movement::Up => println!("Ball Moved Up"),
        Movement::Down => println!("Ball Moved Down"),
        Movement::Left => println!("Ball Moved Left"),
        Movement::Right => println!("Ball Moved Right"),

    }
}

pub fn run(){

    println!("\n******** ENUMS ********");

    let action1 = Movement::Up;
    let action2 = Movement::Down;
    let action3 = Movement::Left;
    let action4 = Movement::Right;
    
    move_ball(action1);
    move_ball(action2);
    move_ball(action3);
    move_ball(action4);
}