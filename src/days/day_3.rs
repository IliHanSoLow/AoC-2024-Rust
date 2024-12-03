use std::fs::*;
use regex::Regex;

pub fn main(){

	let path = "res/day3-input.txt";
    part_1(path);
    part_2(path);
}

fn part_1(path: &str){
    let my_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let hay = read_to_string(path).unwrap();
    // let hay = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let mut results = vec![];

    for (_, [x, y]) in my_regex.captures_iter(&hay).map(|c| c.extract()){
        results.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }

    // println!("{:?}", results);

    let mut solution = 0;
    for (x,y) in results{
		solution += x*y;
    }
    println!("Solution1: {solution}");
}

fn part_2(path: &str){
    let my_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // let exclude_regex = fancy_regex::Regex::new(r"don't\(\)(.*?)(?=(do\(\)|$))/gm").unwrap();
    let exclude_regex_1 = Regex::new(r"(don't\(\).*?do\(\))|(don't\(\).*?$)").unwrap();

    let hay = read_to_string(path).unwrap();
    // let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

    let hay = hay.replace("\n", "");

    let mut results = vec![];
    let new_hay = exclude_regex_1.replace_all(&hay, "hello").to_string();

    for (_, [x, y]) in my_regex.captures_iter(&new_hay).map(|c| c.extract()){
        results.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }

    // println!("{:?}", results);
    println!("{new_hay}");

    let mut solution = 0;
    for (x,y) in results{
		solution += x*y;
    }
    println!("Solution2: {solution}");
}
