use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part1(file_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines_count = reader.lines().count();

    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut figures: Vec<Vec<i64>> = Vec::new();

    let mut first_line = true;

    let mut x = 0;
    for line in reader.lines() {
        let line_string = line?;
        let range_values_string = line_string.split(" ");
        let range_values_collection = range_values_string.collect::<Vec<&str>>();

        let mut y = 0;
        for mut value in range_values_collection {
            value = value.trim();// Trying to account for random multi-spaces in the input
            if value.len() > 0 { // Trying to account for random multi-spaces in the input
                if first_line {
                    figures.push(vec![0; lines_count]);
                }
                
                if value == "*" {
                    figures[y][0] = 1;
                    //value="1";
                } // 1 for multiplication
                else if value == "+" {
                    figures[y][0] = 0;
                    //value="0";
                } // 0 for addition
                else {              
                    let number: i64 = match value.parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Couldn't parse {} as number", value);
                            continue;
                        }
                    };
                    
                    figures[y][x+1] = number; // Does this even work?? Will it panic, there's only one way to find out
                }
                y += 1;
            }
        }

        if first_line {
            first_line = false;
        }
        x += 1;
    }

    let mut totals: Vec<i64> = Vec::new();
    for x in figures {

        if x[0] == 1 {
            totals.push(x.iter().product());
            //println!("Performing sum operation, result = {}", total);
        } else if x[0] == 0 {
            totals.push(x.iter().sum());
            //println!("Performing multiple operation, result = {}", total);
        } else {
            eprintln!("Something has gone wrong, the operation value is neither a 1 or a 0");
            continue;
        }
        
    }

    Ok(totals.iter().sum())
}

pub fn part2(file_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<String> = Vec::new();
    let mut ranges: Vec<(usize,usize)> = Vec::new();

    for line in reader.lines() {
        let line_string = line?;
        grid.push(String::from(&line_string));

        if line_string.chars().nth(0) == Some('*') || line_string.chars().nth(0) == Some('+') {
            // Basically if we're on the last line, we want to calculate the x ranges of each sum
            for x in 0..line_string.len() {
                let cur_char: char = line_string.chars().nth(x).expect("Should be a valid char");
                if cur_char == '*' || cur_char == '+' {
                    let range_len = ranges.len();
                    if ranges.len() > 0 && x > 1 {
                        ranges[range_len-1].1 = x-2;
                    }
                    ranges.push((x,x));
                }
            }
            let range_len = ranges.len();
            ranges[range_len-1].1 = line_string.len()-1;
        }
    }

    let mut totals: Vec<i64> = Vec::new();

    for range in ranges {
        let mut numbers: Vec<i64> = Vec::new();
        let mut operation = '-';

        for x in (range.0..=range.1).rev() {
            let mut number = String::new();
            for y in 0..grid.len() {
                let cur_char: char = grid[y].chars().nth(x).expect("Should be a valid char");
                if cur_char == '*' || cur_char == '+' {
                    operation = cur_char;
                } else if cur_char != ' ' {
                    number.push(cur_char);
                }
            }
            let number_i: i64 = match number.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Couldn't convert {} to number", number);
                    continue;
                }
            };
            numbers.push(number_i);
        }
        if operation == '*' {
            totals.push(numbers.iter().product());
        } else if operation == '+' {
            totals.push(numbers.iter().sum());
        } else {
            eprintln!("No operation was detected, something must've gone wrong");
        }
    }

    Ok(totals.iter().sum())
}