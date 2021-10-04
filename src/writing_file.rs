use std::fs::File;
use std::io::Write;

pub fn run(){
    let mut file = File::create("output.txt").expect("Cannot create a file!!!");
    
    for _i in 1..10{
        file.write_all(_i.to_string().as_bytes()).expect("Cannot write to the file!!!");
        // file.write_all(&u8).expect("Cannot write to the file");
    }
    
}