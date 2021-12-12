use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut grid : Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut row : Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap())
        }
        grid.push(row);
    }
    let low_points : Vec<(u32, u32)> = part1(&grid);
    part2(&grid, &low_points);
}

fn part1(grid : &Vec<Vec<u32>>) -> Vec<(u32, u32)> {
    let mut total_risk = 0;
    let mut low_points : Vec<(u32, u32)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let curr = grid[i][j];
            if i != 0 {
                let above = grid[i-1][j];
                if above <= curr {
                    continue;
                }
            }
            if j != 0 {
                let left = grid[i][j-1];
                if left <= curr {
                    continue;
                }
            }
            if i != grid.len() - 1 {
                let below = grid[i+1][j];
                if below <= curr {
                    continue;
                }
            }
            if j != grid[i].len() - 1 {
                let right = grid[i][j+1];
                if right <= curr {
                    continue;
                }
            }
            total_risk += 1 + curr;
            low_points.push((i as u32, j as u32));
        }
    }
    println!("{}", total_risk);
    return low_points;
}

fn part2(grid : &Vec<Vec<u32>>, low_points : &Vec<(u32, u32)>) {
    let mut basin_sizes : Vec<u32> = Vec::new();
    let mut seen : HashSet<(u32, u32)> = HashSet::new();
    for point in low_points {
        let mut basin_size = 0;
        let mut q : VecDeque<(u32, u32)> = VecDeque::new();
        q.push_back(*point);
        while q.len() != 0 {
            let (x, y) = q.pop_front().unwrap();
            seen.insert((x,y));
            let i = x as usize;
            let j = y as usize;
            let curr = grid[i][j];
            basin_size += 1;
            if i != 0 && grid[i-1][j] != 9 {
                if !seen.contains(&(x-1, y)) && grid[i-1][j] > curr {
                    q.push_back((x-1, y));
                    seen.insert((x-1, y));
                }
            }
            if j != 0 && grid[i][j-1] != 9 {
                if !seen.contains(&(x, y-1)) && grid[i][j-1] > curr {
                    q.push_back((x, y-1));
                    seen.insert((x, y-1));
                }
            }
            if i != grid.len() - 1 && grid[i+1][j] != 9 {
                if !seen.contains(&(x+1, y)) && grid[i+1][j] > curr {
                    q.push_back((x+1, y));
                    seen.insert((x+1, y));
                }
            }
            if j != grid[i].len() - 1 && grid[i][j+1] != 9 {
                if !seen.contains(&(x, y+1)) && grid[i][j+1] > curr {
                    q.push_back((x, y+1));
                    seen.insert((x, y+1));
                }
            }
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!("{:?}", basin_sizes);
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
