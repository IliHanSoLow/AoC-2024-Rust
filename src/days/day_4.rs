use std::fs::*;
use colored::Colorize;

pub fn main() {
    let file = "res/day4-input.txt";
    // part_1(file);
    part_2(file);
}

fn part_1(file: &str){
    let mut solution = 0;

    let test_input = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let matrix: Vec<Vec<char>>;

    let mut tmp_matrix: Vec<Vec<char>> = Vec::new();
    let mut debugger: Vec<Vec<i32>> = Vec::new();
    for l in read_to_string(file).unwrap().lines() {
    // for l in test_input.lines() {
        let mut char_vec: Vec<char> = Vec::new();
        let mut debugger_vec: Vec<i32> = Vec::new();
        for c in l.chars() {
            char_vec.push(c);
            debugger_vec.push(0);
        }
        tmp_matrix.push(char_vec);
        debugger.push(debugger_vec);
    }
    matrix = tmp_matrix;

    for i in 0..matrix.len(){
        for j in 0..matrix[0].len(){
            if matrix[i][j] == 'X'{
                // println!("Found X at {i}, {j}");
                debugger[i][j] = 1;
                solution += find_word(0, (i, j), (i, j), &matrix, &mut debugger);
            }
        }
    }
    /* let i = 1;
    let j = 4;
    debugger[i][j] = 1;
    solution_1 += find_word(0, (i, j), (i, j), &matrix, &mut debugger); */

    println!("Solution1: {solution}");

    for (chars, indecies) in matrix.iter().zip(debugger.iter()) {
        for (i,j) in chars.iter().zip(indecies.iter()){
            let a = match j {
                // 0 => format!("{}", i),
                0 => format!("{}", "."),
                1 => format!("{}", i).red().to_string(),
                2 => format!("{}", i).green().to_string(),
                3 => format!("{}", i).blue().to_string(),
                4 => format!("{}", i).purple().to_string(),
                _ => String::new()
            };
            print!("{}", a);
        }
        print!("\n");
    }
}

fn find_word(index: i32, current: (usize, usize), prev: (usize, usize), matrix: &Vec<Vec<char>>, debugger: &mut Vec<Vec<i32>>) -> i32{
    let (i,j) = current;

    let mut counter = 0;

    let char = match index {
        0 => 'M',
        1 => 'A',
        2 => 'S',
        _ => '0'
    };
    if char == 'M'{
        for k in (0..=2) {
            for l in (0..=2) {
                let k = k as i32 -1;
                let l = l as i32 -1;
                println!("__I+K__{}", i as i32+k);
                println!("__J+L__{}", j as i32+l);
                if !((i as i32+k) as usize >= matrix.len() || (j as i32+l) as usize >= matrix[0].len()){
                    println!("{}", matrix[( i as i32+k ) as usize][( j as i32+l ) as usize]);
                    if matrix[(i as i32+k) as usize][(j as i32+l)as usize] == char{
                        println!("found M at {}, {}", i as i32+k, j as i32+l);
                        println!("found M at i{}, j{}", i, j);
                        println!("found M at current {:?}", current);
                        debugger[( i as i32+k ) as usize][( j as i32+l ) as usize] = index+2;
                        counter += find_word(index+1, (( i as i32+k ) as usize, ( j as i32+l ) as usize), current, matrix, debugger);
                    }
                }
            }
        }
    }
    else {
        let (p1, p2) = prev;
        let k = 2*i as i32 - p1 as i32;
        let l = 2*j as i32 - p2 as i32;
        // let (k,l) = ((i as i32+(i as i32 -prev.0 as i32)*(-1)) as usize,
        // (j as i32+ (j as i32 -prev.1 as i32)*(-1)) as usize);
        println!("{i}, {j}\t{p1}, {p2}\t{k},{l}");
        if !(k as usize >= matrix.len() || l as usize >= matrix[0].len()){
            println!("{}", matrix[k as usize][l as usize]);
            if matrix[k as usize][l as usize] == char{
                debugger[k as usize][l as usize] = index+2;
                println!("Found {} at {}, {}", char, k,l);
                if char == 'S'{
                    // println!("Found {} at {}, {}", char, k,l);
                    counter+=1
                }
                counter += find_word(index+1, (k as usize, l as usize), current, matrix, debugger);
            }
        }
    }

    counter
}

fn part_2(file: &str){
    let matrix: Vec<Vec<char>>;
    let mut solution = 0;

    let test_input = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let mut tmp_matrix: Vec<Vec<char>> = Vec::new();
    let mut debugger: Vec<Vec<i32>> = Vec::new();
    for l in read_to_string(file).unwrap().lines() {
    // for l in test_input.lines() {
        let mut char_vec: Vec<char> = Vec::new();
        let mut debugger_vec: Vec<i32> = Vec::new();
        for c in l.chars() {
            char_vec.push(c);
            debugger_vec.push(0);
        }
        tmp_matrix.push(char_vec);
        debugger.push(debugger_vec);
    }
    matrix = tmp_matrix;

    for i in 0..matrix.len(){
        for j in 0..matrix[0].len(){
            let mut counter = 0;
            if matrix[i][j] == 'A'{
                println!("Found A at {i}, {j}");
                debugger[i][j] = 1;
                for a in [-1,1]{
                    for b in [-1,1]{
                        let ia = i as i32+a;
                        let jb = j as i32+b;
                        if !(ia as usize >= matrix.len() || jb as usize >= matrix[0].len()){
                            if matrix[ia as usize][jb as usize] == 'M' {
                                debugger[ia as usize][jb as usize] = 2;
                                let x = 2*i as i32 - ia;
                                let d = 2*j as i32 - jb;
                                if !(x as usize >= matrix.len() || d as usize >= matrix[0].len()){
                                    if matrix[x as usize][d as usize] == 'S' {
                                        debugger[x as usize][d as usize] = 3;
                                        counter+=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if counter == 2{
                solution+=1
            }
        }
    }




    for (chars, indecies) in matrix.iter().zip(debugger.iter()) {
        for (i,j) in chars.iter().zip(indecies.iter()){
            let a = match j {
                // 0 => format!("{}", i),
                0 => format!("{}", "."),
                1 => format!("{}", i).red().to_string(),
                2 => format!("{}", i).green().to_string(),
                3 => format!("{}", i).blue().to_string(),
                _ => String::new()
            };
            print!("{}", a);
        }
        print!("\n");
    }

    println!("Solution2: {solution}");
}
