fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    println!("\n\n");
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut length : usize = 0;
    let mut num_zeros : Vec<i32> = Vec::new();
    let mut num_ones : Vec<i32> = Vec::new();
    for bitstring in lines {
        if length == 0 {
            length = bitstring.chars().count();
            num_zeros = vec![0; length];
            num_ones = vec![0; length];
        }
        for (i, c) in bitstring.chars().enumerate() {
            if c == '0' {
                num_zeros[i] += 1;
            } else {
                num_ones[i] += 1;
            }
        }
    }

    let mut gamma : String = String::from("");
    let mut epsilon : String = String::from("");

    for i in 0..num_ones.len() {
        if num_zeros[i] > num_ones[i] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    println!("{}", gamma);
    println!("{}", epsilon);

    let int_gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let int_epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", int_gamma * int_epsilon);

    return;
}

fn get_most_common_bit_o(lines : Vec<&str>, index : usize)
    -> char {
    let mut num_zeros = 0;
    let mut num_ones = 0;
    for j in 0..lines.len() {
        if lines[j].chars().nth(index) == Some('0') {
            num_zeros += 1;
        } else {
            num_ones += 1;
        }
    }
    if num_zeros > num_ones {
        return '0';
    } else {
        return '1';
    }
}

fn get_least_common_bit_c(lines : Vec<&str>, index: usize)
    -> char {
    let mut num_zeros = 0;
    let mut num_ones = 0;
    for j in 0..lines.len() {
        if lines[j].chars().nth(index) == Some('0') {
            num_zeros += 1;
        } else {
            num_ones += 1;
        }
    }
    if num_ones < num_zeros {
        return '1';
    } else {
        return '0';
    }
}

fn part2(lines: &Vec<&str>) {

    let mut possible_oxygen = lines.clone();
    let mut index = 0;
    while possible_oxygen.len() != 1 {
        let most_common_bit = get_most_common_bit_o(possible_oxygen.clone(), index);
        possible_oxygen.retain(
            |&s| s.chars().nth(index) == Some(most_common_bit)
        );
        index += 1;

    }

    let mut possible_co2 = lines.clone();
    index = 0;
    while possible_co2.len() != 1 {
        let least_common_bit = get_least_common_bit_c(possible_co2.clone(), index);
        possible_co2.retain(
            |&s| s.chars().nth(index) == Some(least_common_bit)
        );
        index += 1;
    }

    println!("{}", possible_oxygen[0]);
    println!("{}", possible_co2[0]);
    
    let int_oxygen = isize::from_str_radix(&possible_oxygen[0], 2).unwrap();
    let int_co2 = isize::from_str_radix(&possible_co2[0], 2).unwrap();

    println!("{}", int_oxygen * int_co2);

}
