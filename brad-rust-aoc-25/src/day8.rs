use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::f64;
use std::cmp;

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// Answer is too high - kill me now

pub fn part1(file_name: &str, top_num: i32) -> Result<i64, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Building list of co-ordinates from file...");

    let mut coords: Vec<Point3D> = Vec::new();
    for line in reader.lines() {
        let line_string = line?;
        let coord_values_string = line_string.split(",");
        let coord_values_collection = coord_values_string.collect::<Vec<&str>>();

        if coord_values_collection.len() < 3 {
            eprintln!("Line parsed incorrectly, co-ordinate ({}) malformed", line_string);
            continue;
        }

        let x: f64 = match coord_values_collection[0].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Co-ordinate parsed incorrectly, co-ordinate ({}) malformed", line_string);
                continue;
            }
        };
        let y: f64 = match coord_values_collection[1].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Co-ordinate parsed incorrectly, co-ordinate ({}) malformed", line_string);
                continue;
            }
        };
        let z: f64 = match coord_values_collection[2].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Co-ordinate parsed incorrectly, co-ordinate ({}) malformed", line_string);
                continue;
            }
        };

        coords.push(Point3D {x, y, z});
    }

    println!("{} elements added.", coords.len());

    println!("Calculating euclidean distances between all points...");

    let mut connections: Vec<(i64, i64)> = Vec::new();
    let mut distances: Vec<f64> = Vec::new();
    for x in 0..coords.len() {
        for y in 0..coords.len() {
            let min = cmp::min(x, y) as i64;
            let max = cmp::max(x, y) as i64;
            if x == y || connections.contains(&(min, max)) { continue; }
            connections.push((min, max));
            distances.push(euclidean_distance_3d(&coords[min as usize], &coords[max as usize]));
            //distances.push((cmp::min(x as i32, y as i32), cmp::max(x as i32, y as i32), euclidean_distance_3d(&coords[x], &coords[y])));
        }
    }

    println!("Calculated {} distances", distances.len());

    println!("Sorting and truncating list...");

    let mut junction_distances: Vec<((i64, i64), f64)> = connections.into_iter().zip(distances).collect(); // Zip the two vectors together
    junction_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); // Then sort by .1 (distances)

    junction_distances.truncate(top_num as usize); // Take the top 10

    println!("Creating chains from truncated list...");

    let mut chains: Vec<Vec<i64>> = Vec::new();

    'upper: for j in &junction_distances {
        println!("{}-{} {}", j.0.0, j.0.1, j.1);
        for x in 0..chains.len() {
            if chains[x].contains(&j.0.0) && chains[x].contains(&j.0.1) {
                continue 'upper;
            } else if chains[x].contains(&j.0.1) {
                chains[x].push(j.0.0);
                continue 'upper; // This should return to the next j not the next x
            } else if chains[x].contains(&j.0.0) {
                chains[x].push(j.0.1);
                continue 'upper;
            }
        }
        // If it reaches this point no matches were made so we create a new one
        chains.push(vec![j.0.0, j.0.1]);
    }

    // I had a problem whereby if you get a pair that joins two circuits together it was only adding half of the pair to the first matching circuit and NOT joining them together
    // This feels like a warcrime but I think it solves it correctly

    let mut changes = 1;

    while changes > 0 {
        changes = 0;
        'upper: for x in 0..chains.len()-1 {
            for y in x+1..chains.len() {
                for i in 0..chains[y].len() {
                    if chains[x].contains(&chains[y][i]) {
                        chains[y].remove(i);
                        let mut temp_chain = chains[y].clone();
                        chains[x].append(&mut temp_chain);
                        chains.remove(y);
                        changes+=1;
                        continue 'upper;
                    }
                }
            }
        }
    }

    println!("{} chains created.", chains.len());

    println!("Sorting chain lengths to establish top 3...");

    let mut chain_lens: Vec<i64> = Vec::new();
    for chain in &chains {
        chain_lens.push(chain.len() as i64);
        //println!("{}", chain.len())
    }

    chain_lens.sort();
    chain_lens.reverse();
    chain_lens.truncate(3);

    for chain in &chain_lens {
        println!("{}", chain);
    }

    println!("Finding product to return...");

    Ok(chain_lens.iter().product())
}

// Function to calculate the Euclidean distance between two 3D points
fn euclidean_distance_3d(p1: &Point3D, p2: &Point3D) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    let squared_distance = dx * dx + dy * dy + dz * dz;

    squared_distance.sqrt()
}