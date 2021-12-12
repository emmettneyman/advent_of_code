use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut grid : [[u32 ; 10] ; 10] = [[0; 10] ; 10];
    for i in 0..10 {
        for j in 0..10 {
            let c = lines[i].chars().collect::<Vec<char>>()[j];
            grid[i][j] = c.to_digit(10).unwrap();
        }
    }
    part1(grid);
    part2(grid);
}

fn in_range(i : i32, j : i32) -> bool {
    return i < 10 && j < 10 && i >= 0 && j >= 0;
}

fn part1(mut grid : [[u32 ; 10] ; 10]) {
    let mut flashes = 0;
    for _ in 0..100 {
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }
        let mut flash = true;
        let mut flashed : HashSet<(i32, i32)> = HashSet::new();
        while flash {
            flash = false;
            for i in 0..10 {
                for j in 0..10 {
                    let x = i as i32;
                    let y = j as i32;
                    if grid[i][j] > 9 && !flashed.contains(&(x, y)){
                        flash = true;
                        flashed.insert((x, y));
                        flashes += 1;
                        for k in x-1..x+2 {
                            for l in y-1..y+2 {
                                if in_range(k, l) && (k != x || l != y) {
                                    grid[k as usize][l as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        for loc in flashed {
            let i = loc.0 as usize;
            let j = loc.1 as usize;
            grid[i][j] = 0;
        }
    }
    println!("{}", flashes);
}

fn part2 (mut grid : [[u32 ; 10] ; 10]) {
    let mut step = 0;
    loop {
        step += 1;
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }
        let mut flash = true;
        let mut flashed : HashSet<(i32, i32)> = HashSet::new();
        while flash {
            flash = false;
            for i in 0..10 {
                for j in 0..10 {
                    let x = i as i32;
                    let y = j as i32;
                    if grid[i][j] > 9 && !flashed.contains(&(x, y)){
                        flash = true;
                        flashed.insert((x, y));
                        for k in x-1..x+2 {
                            for l in y-1..y+2 {
                                if in_range(k, l) && (k != x || l != y) {
                                    grid[k as usize][l as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if flashed.len() == 100 {
            println!("{}", step);
            break;
        }
        for loc in flashed {
            let i = loc.0 as usize;
            let j = loc.1 as usize;
            grid[i][j] = 0;
        }
        
    }
}
