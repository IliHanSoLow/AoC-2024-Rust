use std::fs::*;
pub fn main(){
    let file_path = "res/day1.txt";

    for line in read_to_string(file_path).unwrap().lines(){
        println!("{}", line);
    }
}
