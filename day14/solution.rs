use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let pattern = lines[0];
    let mut map: HashMap<(char, char), char> = HashMap::new(); 
    for i in 2..lines.len() {
        let line = lines[i];
        let split = line.split(" -> ").collect::<Vec<&str>>();
        let key: Vec<char> = split[0].chars().collect::<Vec<char>>();
        let val: Vec<char> = split[1].chars().collect::<Vec<char>>();
        map.insert((key[0], key[1]), val[0]);
    }
    part1(pattern, &map);
    part2(pattern, &map);
}

fn part1(pattern: &str, map: &HashMap<(char, char), char>) {
    let mut final_pattern: String = pattern.to_string();
    let mut temp_str: String = "".to_string();
    for _ in 0..10 {
        let chars = final_pattern.chars().collect::<Vec<char>>();
        for i in 0..final_pattern.len() - 1 {
            let c = map.get(&(chars[i], chars[i+1])).unwrap();
            temp_str.push(chars[i]);
            temp_str.push(*c);
        }
        temp_str.push(*chars.last().unwrap());
        final_pattern = temp_str;
        temp_str = "".to_string();
    }
    let mut letter_map: HashMap<char, u32> = HashMap::new();
    for c in final_pattern.chars() {
        *letter_map.entry(c).or_insert(0) += 1;
    }
    let mut count_vec: Vec<(&char, &u32)> = letter_map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("{}", count_vec[0].1 - count_vec.last().unwrap().1);
}

// I took the solution algorithm from: github.com/chaosteil/aoc2021/blob/main/aoc14/src/main.rs
fn part2(pattern: &str, map: &HashMap<(char, char), char>) {
    let mut counts: HashMap<(char, char), u64> = HashMap::new();
    let chars = pattern.chars().collect::<Vec<char>>();
    for i in 0..pattern.len() - 1 {
       *counts.entry((chars[i], chars[i+1])).or_default() += 1; 
    }
    for _ in 0..40 {
        let mut curr_count: HashMap<(char, char), u64> = HashMap::new();
        for (key, count) in counts.iter() {
            let new_char = map.get(key).unwrap();
            *curr_count.entry((key.0, *new_char)).or_default() += count;
            *curr_count.entry((*new_char, key.1)).or_default() += count;
        }
        counts = curr_count;
    }
    let mut letter_map: HashMap<char, u64> = HashMap::new();
    for (key, count) in counts.iter() {
        *letter_map.entry(key.0).or_default() += count;
    }
    let final_char = pattern.as_bytes()[pattern.len() - 1] as char;
    *letter_map.entry(final_char).or_default() += 1;
    let mut count_vec: Vec<(&char, &u64)> = letter_map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
     println!("{}", count_vec[0].1 - count_vec.last().unwrap().1);
}
