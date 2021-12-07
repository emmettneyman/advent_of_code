fn main() {
    let input = include_str!("./input.txt");
    let nums : String = input.split_whitespace().collect();
    let fish : Vec<i32> = nums.split(",")
                              .map(|s| s.parse::<i32>().unwrap())
                              .collect();
    loop_days(&fish, 80);
    loop_days(&fish, 256);
}

fn loop_days(fish : &Vec<i32>, days : i32) {
    let mut state_count : Vec<i64> = vec![0,0,0,0,0,0,0,0,0];
    for f in fish {
        state_count[*f as usize] += 1;
    }
    println!("{:?}", state_count);
    for _i in 0..days {
        let new_fish = state_count.remove(0);
        state_count.push(new_fish);
        state_count[6] += new_fish;
    }
    let total_fish : i64 = state_count.iter().sum();
    println!("{}", total_fish);
}
