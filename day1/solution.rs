use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut  depths : Vec<i32> = Vec::new();
    for line in reader.lines() {
        let depth : i32 = line?.parse::<i32>().unwrap();
        depths.push(depth);
    }

    let mut increases : i32 = 0;
    for index in 1..depths.len() {
        if depths[index] > depths[index - 1] {
            increases += 1;
        }
    }
    println!("{}", increases);

    let mut window_increases : i32 = 0;
    let mut prev_window_sum : i32 = depths[0] + depths[1] + depths[2];
    for index in 3..depths.len() {
        let curr_window_sum : i32 = depths[index] +
            depths[index - 1] + depths[index - 2]; 
        if curr_window_sum > prev_window_sum {
            window_increases += 1;
        }
        prev_window_sum = curr_window_sum;
    }
    println!("{}", window_increases);

    Ok(())
}
