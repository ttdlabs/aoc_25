use std::fs;

fn sum_vector(ids: Vec<String>) -> i64 {
    let mut id_sum: i64 =0;
    for id in ids {
        //println!("{}", id);
        let id_int: i64 = match id.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Couldn't parse {} into number", id);
                continue;
            }
        };
        id_sum = id_sum + id_int;
    }
    id_sum
}

pub fn part1(file_name: &str) -> i64 {
    let data = fs::read_to_string(file_name).expect("Should be able to read file");
    
    let ranges = data.split(",");

    let mut ids: Vec<String> = Vec::new();

    for range in ranges {
        let range_values_string = range.split("-");
        let range_values_collection = range_values_string.collect::<Vec<&str>>();

        if range_values_collection.len() < 2 {
            eprintln!("Malformed range: {}", range);
            continue;
        }

        let range_start: i64 = match range_values_collection[0].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Could not parse {} as number", range_values_collection[0]);
                continue;
            }
        };
        let range_end: i64 = match range_values_collection[1].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Could not parse {} as number", range_values_collection[1]);
                continue;
            }
        };

        for n in range_start..range_end+1 {
            //println!("{}", n);
            
            let n_string = n.to_string();
            let n_len = n_string.chars().count() as i32;
            let n_split_poss = n_len/2;


            if n_len % 2 == 0 {
                let first_string = &n_string[0..n_split_poss as usize];
                let second_string = &n_string[n_split_poss as usize..];

                if first_string == second_string {
                    ids.push(n_string);
                }
            }
        }

    }

    sum_vector(ids)
}

pub fn part2(file_name: &str) -> i64 {
    let data = fs::read_to_string(file_name).expect("Should be able to read file");
    
    let ranges = data.split(",");

    let mut ids: Vec<String> = Vec::new();

    for range in ranges {
        let range_values_string = range.split("-");
        let range_values_collection = range_values_string.collect::<Vec<&str>>();

        if range_values_collection.len() < 2 {
            eprintln!("Malformed range: {}", range);
            continue;
        }

        let range_start: i64 = match range_values_collection[0].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Could not parse {} as number", range_values_collection[0]);
                continue;
            }
        };
        let range_end: i64 = match range_values_collection[1].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Could not parse {} as number", range_values_collection[1]);
                continue;
            }
        };

        for n in range_start..range_end+1 {            
            let n_string = n.to_string();
            let n_len = n_string.chars().count() as i32;
            let n_split = n_len/2;
            let mut higher_matching = false;

            //println!("Considering ID: {}", n_string);

            for step in 1..=n_split { // Iterate through possible sub-string lengths
                let mut slices: Vec<&str> = Vec::new();
                if n_len % step == 0 { // Make sure it's divisible
                    for x in (0..n_len).step_by(step as usize) {
                        let end = x+step;
                        let slice = &n_string[x as usize..end as usize];
                        slices.push(slice);
                    }

                    //println!("Generate digit vector: {:?}", slices);
                
                    // Check everything in slice here
                    let mut cur_value = "";
                    let mut matching = true;
                    
                    for number in slices {
                        if cur_value == "" {
                            cur_value = number;
                        }else {
                            if cur_value != number {
                                matching = false;
                            }
                        }
                    }
                    if matching {
                        higher_matching = true;
                    }
                }

            }

            if higher_matching {
                //println!("Value {} repeated", n_string);
                ids.push(n_string);
            }

        }

    }

    sum_vector(ids)
}