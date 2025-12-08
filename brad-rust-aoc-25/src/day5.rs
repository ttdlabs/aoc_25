use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cmp;

pub fn part1(file_name: &str) -> Result<(i32, i64), Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut fresh_ids: Vec<(i64, i64)> = Vec::new();
    let mut fresh_ingest = true;
    let mut fresh_count = 0;
    let mut total_fresh_count = 0;
    let mut overlap_count = 0;

    for line in reader.lines() {
        let line_string = line?;

        if fresh_ingest {
            if line_string.len() == 0 {// 0 length is the separator before we check IDs
                fresh_ingest = false;
                println!("Separator found - moving to checking IDs")
            } else {
                let range_values_string = line_string.split("-");
                let range_values_collection = range_values_string.collect::<Vec<&str>>();

                if range_values_collection.len() < 2 {
                    eprintln!("Malformed range: {}", line_string);
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

                // Add the IDs in the range to the fresh id vector
                fresh_ids.push((range_start, range_end));
                total_fresh_count += range_end-range_start+1;

            }
        } else {
            let mut fresh = false;
            let line_num: i64 = match line_string.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Couldn't parse {} as number", &line_string);
                    continue;
                }
            };
            // This must be an ID to check
            for range in &fresh_ids {
                if line_num >= range.0 && line_num <= range.1 {
                    fresh = true;
                }
            }
            if fresh {
                fresh_count += 1;
            }
        }
    }

    // sort fresh_ids by first elemement then second
    fresh_ids.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let mut merged_ranges: Vec<(i64,i64)> = Vec::new();
    merged_ranges.push(fresh_ids[0]);

    for x in 1..fresh_ids.len() {
        let merged_count = &merged_ranges.len();
        let cur_range = &fresh_ids[x];
        let last_range = &merged_ranges[merged_count-1];

        if cur_range.0 <= last_range.1 + 1 {
            merged_ranges[merged_count-1] = (last_range.0, cmp::max(last_range.1, cur_range.1)); // Essentially, if the current range starts mid previous range, expand the previous range to cover this one
        } else {
            merged_ranges.push(*cur_range);
        }
    }

    let mut total_unique_count: i64 = 0;
    for range in merged_ranges {
        //println!("{}-{}", range.0, range.1);
        total_unique_count += range.1 - range.0 + 1;
    }

    Ok((fresh_count, total_unique_count))
}