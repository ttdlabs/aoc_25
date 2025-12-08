use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part1(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // Apparently file operations don't use the "copy" trait so each subsequent call consumes the previous object
    // The only way to establish the file length is to open the file twice...
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines_count = reader.lines().count();

    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rolls: i32 = 0;

    let mut prev_row = String::new();
    let mut cur_row = String::new();
    let mut next_row = String::new();

    let mut index= 0;

    let lines = reader.lines();

    for line in lines {
        let cur_line = line?;

        let mut adj_rolls: String = String::new();

        //println!("Loop index {}", index.to_string());

        // Due to the edges we need a special case for the first row
        if index == 0 {
            cur_row = cur_line;
        } else if index == 1 {// And as we need the next row we need to always look one row in the past
            next_row = cur_line;

            for x in 0..cur_row.len() {// Consider each position
                // Ignore if not a roll
                if cur_row.chars().nth(x) == Some('.') { continue; }

                adj_rolls = String::new();
                // Gather relevant chars from the current and row
                if x > 0 {
                    adj_rolls.push(cur_row.chars().nth(x-1).expect("Shoud be a valid char"));
                    adj_rolls.push(next_row.chars().nth(x-1).expect("Should be a valid char"));
                }
                if x < (cur_row.len()-1) {
                    adj_rolls.push(cur_row.chars().nth(x+1).expect("Should be a valid char"));
                    adj_rolls.push(next_row.chars().nth(x+1).expect("Should be a valid char"));
                }
                adj_rolls.push(next_row.chars().nth(x).expect("Should be a valid char"));

                // Now sum adjacent rolls
                if sum_rolls(&adj_rolls) < 4 {
                    rolls += 1;
                }
            }
            //println!("Rows processing:");
            //println!("{}", cur_row);
            //println!("{}", next_row);
        } else {
            // This is the default nth loop
            prev_row = cur_row.clone();
            cur_row = next_row.clone();
            next_row = cur_line;

            for x in 0..cur_row.len() {
                if cur_row.chars().nth(x) == Some('.') { continue; }

                adj_rolls = String::new();
                if x > 0 {
                    adj_rolls.push(prev_row.chars().nth(x-1).expect("Shoud be a valid char"));
                    adj_rolls.push(cur_row.chars().nth(x-1).expect("Shoud be a valid char"));
                    adj_rolls.push(next_row.chars().nth(x-1).expect("Should be a valid char"));
                }
                if x < (cur_row.len()-1) {
                    adj_rolls.push(prev_row.chars().nth(x+1).expect("Shoud be a valid char"));
                    adj_rolls.push(cur_row.chars().nth(x+1).expect("Should be a valid char"));
                    adj_rolls.push(next_row.chars().nth(x+1).expect("Should be a valid char"));
                }
                adj_rolls.push(prev_row.chars().nth(x).expect("Should be a valid char"));
                adj_rolls.push(next_row.chars().nth(x).expect("Should be a valid char"));

                // Now sum adjacent rolls
                if sum_rolls(&adj_rolls) < 4 {
                    rolls += 1;
                }
            }
            //println!("Rows processing:");
            //println!("{}", prev_row);
            //println!("{}", cur_row);
            //println!("{}", next_row);
        }

        if index == lines_count-1 {
            // On the last row we need to add a line for counting this as the current row
            prev_row = cur_row.clone();
            cur_row = next_row.clone();

            for x in 0..cur_row.len() {
                if cur_row.chars().nth(x) == Some('.') { continue; }

                adj_rolls = String::new();
                if x > 0 {
                    adj_rolls.push(prev_row.chars().nth(x-1).expect("Shoud be a valid char"));
                    adj_rolls.push(cur_row.chars().nth(x-1).expect("Shoud be a valid char"));
                }
                if x < (cur_row.len()-1) {
                    adj_rolls.push(prev_row.chars().nth(x+1).expect("Shoud be a valid char"));
                    adj_rolls.push(cur_row.chars().nth(x+1).expect("Should be a valid char"));
                }
                adj_rolls.push(prev_row.chars().nth(x).expect("Should be a valid char"));

                // Now sum adjacent rolls
                if sum_rolls(&adj_rolls) < 4 {
                    rolls += 1;
                }
            }
            //println!("Rows processing:");
            //println!("{}", prev_row);
            //println!("{}", cur_row);
        }

        index = index + 1;
        //println!("");
    }

    Ok(rolls)
}

fn sum_rolls(line: &str) -> i32 {
    let mut rolls = 0;
    for char in line.chars() {
        if char == '@' {
            rolls += 1;
        }
    }
    rolls
}

pub fn part2(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {

    // Firstly, we'll need to load the entire file into memory, I'm thinking vector of strings, each string is a line

    // Now we do the main loop
    // Preserve the initial state of the board as a working copy and use another as the annotated copy for next iteration
    // Use logic from part 1 to operate on the working copy, changing @ to x where the roll is accessible on the annotated copy
    // Mark a variable with the number of changes made per loop
    // At the end of the loop, overwrite working copy with the annotated copy

    // If that changes count variable is 0, quit

    let mut board: Vec<String> = Vec::new();

    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let cur_line = line?;
        board.push(cur_line);
    }

    let mut rolls: i32 = 0;
    
    let mut looping = true;
    let mut loop_iterations = 0;

    while looping {
        let mut changed = false; // To track if changes were made this iteration

        let mut annotated_board: Vec<String> = board.clone();

        let mut prev_row = String::new();
        let mut cur_row = String::new();
        let mut next_row = String::new();

        for line_index in 0..board.len() {
            let mut adj_rolls: String = String::new();

            if line_index == 0 {
                cur_row = String::from(&board[line_index]); // I think this should copy the string value without stealing it
            } else if line_index == 1 {
                next_row = String::from(&board[line_index]);

                for row_index in 0..cur_row.len() {
                    if cur_row.chars().nth(row_index) == Some('.') { continue; }

                    adj_rolls = String::new();

                    if row_index > 0 {
                        adj_rolls.push(cur_row.chars().nth(row_index-1).expect("Should be a char"));
                        adj_rolls.push(next_row.chars().nth(row_index-1).expect("Should be a char"));
                    }
                    if row_index < (cur_row.len()-1) {
                        adj_rolls.push(cur_row.chars().nth(row_index+1).expect("Should be a char"));
                        adj_rolls.push(next_row.chars().nth(row_index+1).expect("Should be a char"));
                    }
                    adj_rolls.push(next_row.chars().nth(row_index).expect("Should be a char"));

                    if sum_rolls(&adj_rolls) < 4 {
                        // Indicating an accessible roll
                        changed = true;
                        rolls += 1;
                        annotated_board[line_index-1].replace_range(row_index..row_index+1, "x");
                    }                    
                }
            } else {
                prev_row = cur_row.clone();
                cur_row = next_row.clone();
                next_row = String::from(&board[line_index]);

                for row_index in 0..cur_row.len() {
                    if cur_row.chars().nth(row_index) == Some('.') { continue; }

                    adj_rolls = String::new();
                    if row_index > 0 {
                        adj_rolls.push(prev_row.chars().nth(row_index-1).expect("Should be a char"));
                        adj_rolls.push(cur_row.chars().nth(row_index-1).expect("Should be a char"));
                        adj_rolls.push(next_row.chars().nth(row_index-1).expect("Should be a char"));
                    }
                    if row_index < (cur_row.len()-1) {
                        adj_rolls.push(prev_row.chars().nth(row_index+1).expect("Should be a char"));
                        adj_rolls.push(cur_row.chars().nth(row_index+1).expect("Should be a char"));
                        adj_rolls.push(next_row.chars().nth(row_index+1).expect("Should be a char"));
                    }
                    adj_rolls.push(prev_row.chars().nth(row_index).expect("Should be a char"));
                    adj_rolls.push(next_row.chars().nth(row_index).expect("Should be a char"));

                    if sum_rolls(&adj_rolls) < 4 {
                        changed = true;
                        rolls += 1;
                        annotated_board[line_index-1].replace_range(row_index..row_index+1, "x");
                    }
                }
            }

            if line_index == board.len()-1 {
                prev_row = cur_row.clone();
                cur_row = next_row.clone();

                for row_index in 0..cur_row.len() {
                    if cur_row.chars().nth(row_index) == Some('.') { continue; }

                    adj_rolls = String::new();
                    if row_index > 0 {
                        adj_rolls.push(prev_row.chars().nth(row_index-1).expect("Should be a char"));
                        adj_rolls.push(cur_row.chars().nth(row_index-1).expect("Should be a char"));
                    }
                    if row_index < (cur_row.len()-1) {
                        adj_rolls.push(prev_row.chars().nth(row_index+1).expect("Should be a char"));
                        adj_rolls.push(cur_row.chars().nth(row_index+1).expect("Should be a char"));
                    }
                    adj_rolls.push(prev_row.chars().nth(row_index).expect("Should be a char"));

                    if sum_rolls(&adj_rolls) < 4 {
                        changed = true;
                        rolls += 1;
                        annotated_board[line_index].replace_range(row_index..row_index+1, "x");
                    }
                }
            }
        }

        // TESTING CODE
        let testing = false;
        if testing {
            let max_loops = 15;
            if loop_iterations == max_loops { looping = false; }
            println!("This iterations board result:");
            for x in 0..annotated_board.len() {
                println!("{}", &annotated_board[x]);
            }
            println!("");
        }

        // If no changes were made, quit
        if !changed {
            looping = false;
        }
        loop_iterations += 1;

        for x in 0..annotated_board.len() {
            annotated_board[x] = annotated_board[x].replace("x", "."); // Clean up the x's for the next iteration
        }
        board = annotated_board.clone();
    }

    Ok(rolls)
}