// use std::env
use std::fs::File;
use std::io::Write;

pub fn run(){
    // creating vector and collect the arguments passed
    let todo_list: Vec<String> = std::env::args().collect();

    // first variable is always the binary program name
    let program_name = todo_list[0].clone();
    println!("Program Executed: {}", program_name);

    println!("{:?}", todo_list);

    File::create("output.txt").expect("Cannot create a file!!!");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.txt")
        .unwrap();

    for _i in todo_list.iter().skip(1){
        file.write_all(_i.to_string().as_bytes()).expect("Cannot write to the file");
    }

    for todo in std::env::args().skip(1){
        println!("Argument: {}", todo);
    }

    println!("Completed!!!")

}