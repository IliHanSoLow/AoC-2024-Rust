use std::fs::*;

pub fn main(){
	let mut solution_1 = 0;
    let path = "res/day2-input.txt".to_string();
    // let path = "res/day2-test.txt".to_string();
    // let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string(path).unwrap().lines(){
        let mut report: Vec<i32> = Vec::new();
        line.split_whitespace().map(|x| x.to_string().parse::<i32>().unwrap()).for_each(|x| report.push(x)) ;
        solution_1 += check_report(report, false);
    }
    println!("Solution1: {solution_1}");
}

fn check_report(report: Vec<i32>, removed: bool) -> i32{
    let mut solution_1 = 0;
    enum ParsingMode {Inc, Dec, Default}
    let mut current_mode: ParsingMode = ParsingMode::Default;
    let mut counter = 0;
    for (i, _) in report.clone().into_iter().enumerate(){
        if !removed {
            println!("Inner ----");
            let mut new_rep = report.clone();
            new_rep.remove(i);
            solution_1 += check_report(new_rep, true);
            println!("solutionIntern {}", solution_1);
            println!("Outer ----");
        }
        if i == 0{
            if report[i] < report[i+1]{
                println!("{:?}", report);
                println!("Inc");
                current_mode = ParsingMode::Inc;
            }
            else if report[i] > report[i+1]{
                println!("{:?}", report);
                println!("Dec");
                current_mode = ParsingMode::Dec;
            }
        }
        else {
            match current_mode {
                ParsingMode::Inc => {
                    if (report[i-1] >= report[i]) || (report[i]-report[i-1]>3){
                        continue;
                    }
                    counter += 1;
                    if i == report.len()-1{
                        println!("{counter}");
                        if counter >= report.len()-1{
                            solution_1 = 1;
                        }
                    }
                }
                ParsingMode::Dec => {
                    if (report[i-1] <= report[i]) || (report[i-1]-report[i]>3){
                        continue;
                    }
                    counter += 1;
                    if i == report.len()-1{
                        println!("{counter}");
                        if counter >= report.len()-1{
                            solution_1 += 1;
                        }
                    }
                }
                ParsingMode::Default => break,
            }
        }
    }
    if solution_1>1{
        solution_1 = 1;
    }
    println!("solutionFunction {}", solution_1);
    solution_1
}
