use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut depth : i32 = 0;
    let mut hor : i32 = 0;
    let mut aim : i32 = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_str : String = line?;
        let split = line_str.split(" ").collect::<Vec<&str>>();
        let offset = split[1].parse::<i32>().unwrap();
        if split[0] == "forward" {
            hor += offset;
            depth += aim * offset;
        } else if split[0] == "up" {
            aim -= offset;
        } else if split[0] == "down" {
            aim += offset;
        }
    }

    let result = depth * hor;
    println!("{}", result);
    Ok(())

}
