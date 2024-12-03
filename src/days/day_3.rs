use std::fs::*;

pub fn main(){

	let path = "res/day3-input.txt";
    // part_1(path);
    part_2(path);
}

fn part_1(path: &str){
    let my_regex = regex::RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)").multi_line(true).build().unwrap();
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
    let my_regex = regex::RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)").multi_line(true).build().unwrap();
    let exclude_regex = fancy_regex::Regex::new(r"don't\(\)(.*?)(?=(do\(\)|$))/gm").unwrap();

    let hay = read_to_string(path).unwrap();
    // let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

    let mut new_hay = hay.clone();
    while exclude_regex.find(&new_hay).unwrap().is_some(){
    	new_hay = exclude_regex.replace(&new_hay, "imanidentifier").to_string();
    }

    // println!("{hay}");
    // println!("{new_hay}");

    let mut results = vec![];

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
