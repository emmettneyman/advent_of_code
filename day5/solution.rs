use std::cmp;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut ranges = Vec::new();
    for line in lines {
        let split : Vec<&str> = line.split(" -> ").collect();
        let start : Vec<&str> = split[0].split(",").collect();
        let finish : Vec<&str> = split[1].split(",").collect();
        let x1 = start[0].parse::<i32>().unwrap();
        let x2 = finish[0].parse::<i32>().unwrap();
        let y1 = start[1].parse::<i32>().unwrap();
        let y2 = finish[1].parse::<i32>().unwrap();
        ranges.push(((x1, y1),(x2, y2)));
    }
    part1(&ranges);
    part2(&ranges);
}

fn part1(ranges : &Vec<((i32, i32),(i32, i32))>) {
    let mut grid = [[0; 1000] ; 1000];
    for range in ranges {
        let x1 = range.0.0;
        let y1 = range.0.1;
        let x2 = range.1.0;
        let y2 = range.1.1;
        if x1 == x2 || y1 == y2 {
            for i in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
                for j in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
                    grid[i as usize][j as usize] += 1;
                }
            }
        }
    }
    // println!("{:?}", grid);
    let mut hotspots = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] > 1 {
                hotspots += 1;
            }
        }
    }
    println!("{}", hotspots);
}

fn part2(ranges : &Vec<((i32, i32),(i32, i32))>) {
    let mut grid = [[0; 1000] ; 1000];
    for range in ranges {
        let x1 = range.0.0;
        let y1 = range.0.1;
        let x2 = range.1.0;
        let y2 = range.1.1;
        // First handle straight lines
        if x1 == x2 || y1 == y2 {
            for i in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
                for j in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
                    grid[i as usize][j as usize] += 1;
                }
            }
        } else { // Now handle diagonals
            if x1 < x2 && y1 < y2 {
                for i in 0..x2 - x1 + 1 {
                    grid[(x1 + i) as usize][(y1 + i) as usize] += 1;
                }
            } else if x1 > x2 && y1 > y2 {
                for i in 0..x1 - x2 + 1 {
                    grid[(x2 + i) as usize][(y2 + i) as usize] += 1;
                }
            } else if x1 > x2 && y1 < y2 {
                for i in 0..x1 - x2 + 1 {
                    grid[(x1 - i) as usize][(y1 + i) as usize] += 1;
                }
            } else if x1 < x2 && y1 > y2 {
                for i in 0..x2 - x1 + 1 {
                    grid[(x1 + i) as usize][(y1 - i) as usize] += 1;
                }
            }
        }
    }
    // println!("{:?}", grid);
    let mut hotspots = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] > 1 {
                hotspots += 1;
            }
        }
    }
    println!("{}", hotspots);
}
