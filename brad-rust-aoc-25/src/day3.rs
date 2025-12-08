use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


pub fn part1(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut joltages: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let bank = line?;
        let mut top_value = 0; // These defaults work on the assumption that the values range from 1-9
        let mut top_index = 0;

        // First run for top value
        for i in 0..bank.len()-1 {
            let jolt = string_index_to_int(&bank, i);

            if jolt > top_value {
                top_value = jolt;
                top_index = i;
            }
        }
        let mut second_value = 0;

        for i in top_index+1..bank.len() {
            let jolt = string_index_to_int(&bank, i);

            if jolt > second_value {
                second_value = jolt;
            }
        }
        let big_jolt = top_value.to_string() + &second_value.to_string();
        let big_jolt_num: i32 = match big_jolt.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Couldn't parse {} as a number", big_jolt);
                0
            }
        };

        joltages.push(big_jolt_num);
    }
    Ok(joltages.iter().sum())
}

pub fn part2(file_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut joltages: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let bank = line?;
        let mut top_values: Vec<i32> = Vec::new();
        let mut prev_index = 0;

        for n in 0..12 {
            top_values.push(0);
            let start_index = if n == 0 { 0 } else { prev_index+1 };
            let end_index: usize = bank.len()-(11-n);
            for i in start_index..end_index {
                let jolt = string_index_to_int(&bank, i);
                if jolt > top_values[n] {
                    top_values[n] = jolt;
                    prev_index = i;
                }
            }
        }
        let mut big_jolt = String::new();
        for jolt in &top_values {
            big_jolt.push_str(&jolt.to_string());
        }
        let big_jolt_num: i64 = match big_jolt.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Could not parse big jolt number {}", big_jolt);
                0
            }
        };
        joltages.push(big_jolt_num);
    }
    Ok(joltages.iter().sum())
}

fn string_index_to_int(string: &str, index: usize) -> i32 {
    // Convoluted way to turn an indexed charater to an int
    let char_string = string.chars().nth(index).expect("Should be a char").to_string();
    let char_int: i32 = match char_string.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Could not parse {} as a number", char_string);
            0
        }
    };
    char_int
}