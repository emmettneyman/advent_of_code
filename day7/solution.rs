fn main() {
    let input = include_str!("./input.txt");
    let nums : String = input.split_whitespace().collect();
    let crab_positions : Vec<i32> = nums.split(",")
                                        .map(|s| s.parse::<i32>().unwrap())
                                        .collect();
    part1(&crab_positions);
    part2(&crab_positions);
}

fn part1(crab_positions : &Vec<i32>) {
    let max = *crab_positions.iter().max().unwrap();
    let mut  fuel_totals : Vec<i32> = Vec::new();
    for i in 0..max {
        let mut sum = 0;
        for pos in crab_positions.iter() {
            sum += i32::abs(pos - i);
        }
        fuel_totals.push(sum);
    }
    let min_fuel_total = *fuel_totals.iter().min().unwrap();
    println!("{}", min_fuel_total);
}

fn part2(crab_positions : &Vec<i32>) {
    let max = *crab_positions.iter().max().unwrap();
    let mut  fuel_totals : Vec<i32> = Vec::new();
    for i in 0..max {
        let mut sum = 0;
        for pos in crab_positions.iter() {
            let diff = i32::abs(pos - i);
            sum += diff * (diff + 1) / 2;
        }
        fuel_totals.push(sum);
    }
    let min_fuel_total = *fuel_totals.iter().min().unwrap();
    println!("{}", min_fuel_total);
}
