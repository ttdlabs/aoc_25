use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part2(file_name: &str, approach: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // Specify the path to the text file
    let file_path = Path::new(file_name);

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let result: i32;

    if approach == "brute" {
        result = brute_approach(reader)?;
    } else if approach == "maths" {
        result = math_approach(reader)?;
    } else {
        eprintln!("Not a valid approach");
        result = 0;
    }

    Ok(result)
}

pub fn part1(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // Specify the path to the text file
    let file_path = Path::new(file_name);

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut position = 50;
    let mut zero_count = 0;

    // Read each line from the file
    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim();  // Remove leading and trailing whitespace
        
        // Extract the direction character (R or L)
        let direction = &line[0..1];
        // Extract the number of moves
        let number: i32 = match line[1..].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Warning: could not parse \"{}\" as a number.", &line[1..]);
                continue;
            }
        };

        if direction == "R" {
            position = position + number;
        } else if direction == "L" {
            position = position - number;
        } else {
            eprintln!("Direction didn't match expected options");
        }
        //position = position % 100; In Rust % represents remainder not modulo, have to use a method instead
        position = position.rem_euclid(100);

        if position == 0 {
            zero_count = zero_count + 1;
        }

        //println!("{} -> {}    ({})", line, position, zero_count);
    }

    //println!("The password is {}", zero_count);

    Ok(zero_count)
}


fn brute_approach(file_reader: BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {

    let mut zero_count = 0;
    let mut position = 50;

    for line_result in file_reader.lines() {
        let line = line_result?;
        let line = line.trim();

        let direction = &line[0..1];
        let mut number: i32 = match line[1..].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Warning: could not parse \"{}\" as a number", &line[1..]);
                continue;
            }
        };

        if direction == "R" {
            while number > 0 {
                position = position + 1;
                if position == 100 {
                    zero_count = zero_count + 1;
                    position = 0;
                }
                number = number - 1;
            }
        } else if direction == "L" {
            while number > 0 {
                position = position - 1;
                if position == 0 {
                    zero_count = zero_count + 1;
                } else if position == -1 {
                    position = 99;
                }
                number = number - 1;
            }
        }
        //println!("{} -> {}    ({})", line, position, zero_count);
    }

    Ok(zero_count)
}

fn math_approach(file_reader: BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut position = 50;
    let mut new_position;
    let mut zero_count = 0;

    // Read each line from the file
    for line_result in file_reader.lines() {
        let line = line_result?;
        let line = line.trim();  // Remove leading and trailing whitespace
        
        // Extract the direction character (R or L)
        let direction = &line[0..1];
        // Extract the number of moves
        let number: i32 = match line[1..].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Warning: could not parse \"{}\" as a number.", &line[1..]);
                continue;
            }
        };



        // Calculate complete loops (every 100 steps = 1 crossing)
        zero_count = zero_count + number / 100;

        // Calculate the remainder after complete loops
        let remainder = number % 100;

        if direction == "R" {
            new_position = position + remainder;
            if new_position >= 100 {
                zero_count = zero_count + 1;
                new_position = new_position - 100;
            }
        } else {
            new_position = position - remainder;
            // Count if we pass through or land on 0 (but not if we start at 0)
            if new_position <= 0 && position > 0 {
                zero_count = zero_count + 1;
            }
            new_position = new_position.rem_euclid(100);
        }

        position = new_position;


        // I don't think this is necessary but maybe verify
        //if position == 0 {
        //    zero_count = zero_count + 1;
        //}        

        //println!("{} -> {}    ({})", line, position, zero_count);
    }
    Ok(zero_count)
}