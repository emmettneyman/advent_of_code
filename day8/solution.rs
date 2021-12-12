use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines : &Vec<&str>) {
    let mut sum = 0;
    for line in lines.iter() {
        let parts : Vec<&str> = line.split(" | ").collect();
        let output_digits : Vec<&str> = parts[1].split(" ").collect();
        for digit in output_digits.iter() {
            if digit.len() == 2 || digit.len() == 4 ||
               digit.len() == 3 || digit.len() == 7 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

fn part2(lines : &Vec<&str>) {
    for line in lines.iter() {
        let mut segments = ['.', '.', '.', '.', '.', '.', '.'];
        let parts : Vec<&str> = line.split(" | ").collect();
        let output_digits : Vec<&str> = parts[1].split(" ").collect();
        let signal_digits : Vec<&str> = parts[0].split(" ").collect();
        let seven_signal : HashSet<char> =
            HashSet::from_iter(
                signal_digits.iter()
                .filter(|&s| s.len() == 3)
                .collect::<Vec<_>>()[0]
                .chars()
                .collect::<Vec<_>>());
        let one_signal : HashSet<char> =
            HashSet::from_iter(
                signal_digits.iter()
                .filter(|&s| s.len() == 2)
                .collect::<Vec<_>>()[0]
                .chars()
                .collect::<Vec<_>>());
        let four_signal : HashSet<char> =
            HashSet::from_iter(
                signal_digits.iter()
                .filter(|&s| s.len() == 4)
                .collect::<Vec<_>>()[0]
                .chars()
                .collect::<Vec<_>>());
        // This figures out the top segment: a
        segments[0] = *seven_signal.symmetric_difference(&one_signal)
                                   .into_iter().collect::<Vec<_>>()[0];
        let six_nine_signals : Vec<&str> = signal_digits.iter()
            .filter(|&s| s.len() == 6)
            .map(|s| *s)
            .collect();
        let six_nine_a : HashSet<char> = HashSet::from_iter(
            six_nine_signals[0].chars().collect::<Vec<_>>());
        let six_nine_b : HashSet<char> = HashSet::from_iter(
            six_nine_signals[1].chars().collect::<Vec<_>>());
        if one_signal.is_subset(&six_nine_a) {
            // a is 9, b is 6
            for c in one_signal.iter() {
                if !six_nine_b.contains(c) {
                    segments[2] = *c;
                } else {
                    segments[5] = *c;
                }
            }
        } else {
            // b is 9, a is 6
            for c in one_signal.iter() {
                if !six_nine_b.contains(c) {
                    segments[2] = *c;
                } else {
                    segments[5] = *c;
                }
            }
        }
    }
}
