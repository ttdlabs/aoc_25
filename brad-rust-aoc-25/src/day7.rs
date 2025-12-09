use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part1(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    calculate_splits(file_name, false)
}

pub fn part2(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    calculate_universes_alt(file_name)
}

fn calculate_universes_alt(file_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // This works in theory but it's far too slow at the input size to be realistic (took nearly an hour to reach iteration 85 our of 140)
    // https://www.reddit.com/r/adventofcode/comments/1pgnmou/2025_day_7_lets_visualize/
    // Some guidance in this reddit post
    
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut beam_xs: Vec<i32> = Vec::new();
    beam_xs.push(0);

    // Load the original into memory
    let mut orig_grid: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line_string = line?;
        orig_grid.push(line_string);
    }
    let y_max = orig_grid.len();
    let x_max = orig_grid[0].len();

    println!("Processing the consideration for infinite quantum universes:");

    for y in 0..y_max { // At the top level we're approaching this one line at a time
        let z_max = beam_xs.len();

        let new_line = &orig_grid[y];

        for z in 0..z_max { // Then we consider each possible grid
            for x in 0..x_max { // With each character
                let cur_char = new_line.chars().nth(x).expect("Should be a char");

                if cur_char == 'S' {
                    beam_xs[z] = x as i32;
                } else if beam_xs[z] == x as i32 {
                    if cur_char == '^' {
                        beam_xs[z] = (x-1) as i32;
                        beam_xs.push((x+1) as i32);
                    }
                }
            }
        }
        println!("Completed row {}, {} universes generated", y, beam_xs.len());
    }
    Ok(beam_xs.len() as i32)
}

fn calculate_splits(file_name: &str, considering_the_inifite_quantum_universe: bool) -> Result<i32, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<String> = Vec::new();
    let mut beam_xs: Vec<i32> = Vec::new();
    let mut splits = 0;

    for line in reader.lines() {
        let mut line_string = line?;

        for x in 0..line_string.len() as i32 {
            let cur_char = line_string.chars().nth(x as usize).expect("Should be a valid char");
            if cur_char == '.' && beam_xs.contains(&x) {
                line_string.replace_range(x as usize..x as usize+1, "|");
            } else if cur_char == 'S' {
                beam_xs.push(x);
            } else if cur_char == '^' {
                if beam_xs.contains(&x) {
                    splits += 1;

                    beam_xs.remove(beam_xs.iter().position(|&r| r == x).expect("Should be an int."));

                    //let mut temp_value: Option<usize> = Some(0);
                    //while temp_value != None {
                    //    temp_value = beam_xs.iter().position(|&r| r == x);
                    //    if temp_value != None { beam_xs.remove(temp_value.expect("Should be an int.")); }
                    //}

                    if !beam_xs.contains(&(x-1)) || considering_the_inifite_quantum_universe { beam_xs.push(x-1); }
                    if !beam_xs.contains(&(x+1)) || considering_the_inifite_quantum_universe { beam_xs.push(x+1); }
                }
            }
        }
        //println!("{}({})", line_string, splits);
        grid.push(line_string);
    }

    Ok(splits)
}