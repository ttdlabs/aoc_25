mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
use std::time::Instant;
use std::env;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    let mut day = &String::from("8");
    let mut day1_approach = &String::from("maths");

    if args.len() > 1 {
        day = &args[1];
    }
    if args.len() > 2 {
        day1_approach = &args[2];
    }

    if day == "1" {
        let mut now = Instant::now();
        let day1part1 = day1::part1("01.txt")?;
        let day1part1time = now.elapsed();
        
        now = Instant::now();
        let day1part2 = day1::part2("01.txt", &day1_approach)?;
        let day1part2time = now.elapsed();

        println!("Day One Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day1part1, day1part1time);
        println!("Part 2 Answer: {} ({:.2?})", day1part2, day1part2time);
    } else if day == "2" {
        let mut now = Instant::now();
        let day2part1 = day2::part1("02.txt");
        let day2part1time = now.elapsed();

        now = Instant::now();
        let day2part2 = day2::part2("02.txt");
        let day2part2time = now.elapsed();

        println!("Day Two Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day2part1, day2part1time);
        println!("Part 2 Answer: {} ({:.2?})", day2part2, day2part2time);
    } else if day == "3" {
        let mut now = Instant::now();
        let day3part1 = day3::part1("03.txt")?;
        let day3part1time = now.elapsed();

        now = Instant::now();
        let day3part2 = day3::part2("03.txt")?;
        let day3part2time = now.elapsed();

        println!("Day Three Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day3part1, day3part1time);
        println!("Part 2 Answer: {} ({:.2?})", day3part2, day3part2time);
    } else if day == "4" {
        let mut now = Instant::now();
        let day4part1 = day4::part1("04.txt")?;
        let day4part1time = now.elapsed();

        now = Instant::now();
        let day4part2 = day4::part2("04.txt")?;
        let day4part2time = now.elapsed();

        println!("Day Four Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day4part1, day4part1time);
        println!("Part 2 Answer: {} ({:.2?})", day4part2, day4part2time);
    } else if day == "5" {
        let now = Instant::now();
        let day5part1 = day5::part1("05.txt")?;
        let day5part1time = now.elapsed();

        println!("Day Five Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day5part1.0, day5part1time);
        println!("Part 2 Answer: {}", day5part1.1);
    } else if day == "6" {
        let mut now = Instant::now();
        let day6part1 = day6::part1("06.txt")?;
        let day6part1time = now.elapsed();

        now = Instant::now();
        let day6part2 = day6::part2("06.txt")?;
        let day6part2time = now.elapsed();

        println!("Day Six Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day6part1, day6part1time);
        println!("Part 2 Answer: {} ({:.2?})", day6part2, day6part2time);
    } else if day == "7" {
        let mut now = Instant::now();
        let day7part1 = day7::part1("07.txt")?;
        let day7part1time = now.elapsed();

        now = Instant::now();
        let day7part2 = day7::part2("07.txt")?;
        let day7part2time = now.elapsed();

        println!("Day Seven Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day7part1, day7part1time);
        println!("Part 2 Answer: {} ({:.2?})", day7part2, day7part2time);
    } else if day == "8" {
        let mut now = Instant::now();
        let day8part1 = day8::part1("08.txt", 1000)?;
        let day8part1time = now.elapsed();

        now = Instant::now();
        let day8part2 = day8::part2("08.txt")?;
        let day8part2time = now.elapsed();

        println!("Day Eight Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day8part1, day8part1time);
        println!("Part 2 Answer: {} ({:.2?})", day8part2, day8part2time);
    } else if day == "9" {
        let now = Instant::now();
        let day9part1 = day9::part1("09.txt")?;
        let day9part1time = now.elapsed();

        //now = Instant::now();
        //let day9part2 = day9::part2("09b.txt")?;
        //let day9part2time = now.elapsed();

        println!("Day Nine Task Results:");
        println!("Part 1 Answer: {} ({:.2?})", day9part1, day9part1time);
        //println!("Part 2 Answer: {} ({:.2?})", day9part2, day9part2time);
    }

    Ok(())
}
