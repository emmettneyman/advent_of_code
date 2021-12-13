use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut dots: Vec<(u32, u32)> = Vec::new();
    let mut folds: Vec<(char, u32)> = Vec::new();
    for line in lines {
        if line.contains(",") {
            let split: Vec<&str> = line.split(",").collect();
            let x = split[0].parse::<u32>().unwrap();
            let y = split[1].parse::<u32>().unwrap();
            dots.push((x, y));
        } else if line != "" {
            let split: Vec<&str> = line.split(" ").collect();
            let split2: Vec<&str> = split[2].split("=").collect();
            let var = split2[0].chars().collect::<Vec<char>>()[0];
            let val = split2[1].parse::<u32>().unwrap();
            folds.push((var, val));
        }
    }
    part1(&dots, folds[0]);
    part2(&dots, &folds);
}

fn part1(d: &Vec<(u32, u32)>, fold: (char, u32)) {
    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    if fold.0 == 'x' {
        for dot in d {
            let x = dot.0;
            let y = dot.1;
            if x > fold.1 {
                let new_dot = (fold.1 - (x - fold.1), y);
                dots.insert(new_dot);
            } else {
                dots.insert(*dot);
            }
        }
    } else if fold.0 == 'y' {
        for dot in d {
            let x = dot.0;
            let y = dot.1;
            if y > fold.1 {
                let new_dot = (x, fold.1 - (y - fold.1));
                dots.insert(new_dot);
            } else {
                dots.insert(*dot);
            }
        }
    }
    println!("{}", dots.len());
}

fn part2(dots: &Vec<(u32, u32)>, folds: &Vec<(char, u32)>) {
    let mut xmax: usize = 0;
    let mut ymax: usize  = 0;
    
    for dot in dots {
        let x = dot.0 as usize;
        let y = dot.1 as usize;
        if x > xmax {
            xmax = x;
        }
        if y > ymax {
            ymax = y;
        }
    }
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; xmax + 1]; ymax + 1];
    for dot in dots {
        let x = dot.0 as usize;
        let y = dot.1 as usize;
        grid[y][x] = '@';
    }
    for fold in folds {
        let axis = fold.0;
        let val = fold.1 as usize;
        for j in 0..grid.len() {
            for i in 0..grid[j].len() {
                let x = i as usize;
                let y = j as usize;
                if axis == 'x' {
                    if grid[y][x] == '@' && x > val {
                        grid[y][x] = '.';
                        grid[y][val - (x - val)] = '@';
                    }
                    xmax = val;
                } else if axis == 'y' {
                    if grid[y][x] == '@' && y > val {
                        grid[y][x] = '.';
                        grid[val - (y - val)][x] = '@';
                    }
                    ymax = val;
                }
            }
        }
    }
    for i in 0..ymax {
        let line: Vec<char> = grid[i][0..xmax].to_vec();
        println!("{}", line.into_iter().collect::<String>());
    }
}
