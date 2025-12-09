use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part1(file_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut points: Vec<(i64, i64)> = Vec::new();

    // Start by parsing the file string lines into a vector of co-ordinates
    for line in reader.lines() {
        let line_string = line?;

        let coord_values_string = line_string.split(",");
        let coord_values_collection = coord_values_string.collect::<Vec<&str>>();
        let x: i64 = match coord_values_collection[0].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed to parse {} as number", line_string);
                continue;
            }
        };
        let y: i64 = match coord_values_collection[1].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed to parse {} as number", line_string); 
                continue;
            }
        };
        points.push((x, y));
    }

    let mut largest_rec: (i64, i64, i64) = (0, 0, 0); // Better store those x and y co-ords for literally no reason

    for i in 0..points.len() {
        let p1 = &points[i];
        let mut p2s: Vec<(i64,i64,i64)> = Vec::new();
        for j in 0..points.len() {
            if points[j].0 > p1.0 && points[j].1 > p1.1 {
                // Who said we wouldn't need geometry after school
                let p2 = points[j].clone();
                p2s.push((p2.0, p2.1, (p2.0-p1.0+1)*(p2.1-p1.1+1))); // Push it real good
            } else if points[j].0 < p1.0 && points[j].1 > p1.1 {
                let p2 = points[j].clone();
                p2s.push((p2.0, p2.1, (p1.0-p2.0+1)*(p2.1-p1.1+1)));
            }
        }
        if p2s.len() == 0 { continue; } // Ain't nobody love p1
        p2s.sort_by(|a, b| b.2.cmp(&a.2));

        if p2s[0].2 > largest_rec.2 { largest_rec = p2s[0]; } // The king is dead, long live the king

    }

    Ok(largest_rec.2)
}
