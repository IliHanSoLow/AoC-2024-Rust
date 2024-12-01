use std::fs::*;
pub fn main(){
    let file_path = "res/day1-input.txt";

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut solution_1: i32 = 0;

    for line in read_to_string(file_path).unwrap().lines(){
        let parts = line.split("   ").collect::<Vec<&str>>();
        left.push(parts[0].to_string().parse::<i32>().unwrap());
        right.push(parts[1].to_string().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    // Task 1
    for (i, _) in left.iter().enumerate(){
        let dif = left[i]-right[i];
        solution_1 += dif.abs();
    }

    println!("Task 1: {solution_1}");

    // Task 2
    let mut solution_2: i32 = 0;
    let mut last_number = 0;
    let mut last_counter = 0;
    for i in left{
        if last_number == i {
            solution_2 += last_counter*i;
        } else {
            let mut counter = 0;
            for j in &right {
                if i < *j {
                    break;
                }
                if i == *j {
                    counter += 1;
                }
            }
            solution_2 += counter*i;
            last_counter = counter;
        }
        last_number = i;
    }
    println!("Task2: {solution_2}");
}
