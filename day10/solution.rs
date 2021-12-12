fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let incomplete_lines : Vec<&str> = part1(&lines);
    part2(&incomplete_lines);
}

fn part1<'a>(lines : &'a Vec<&str>) -> Vec<&'a str> {
    let mut score : u32 = 0;
    let mut incomplete_lines : Vec<&str> = Vec::new();
    for line in lines.iter() {
        let mut stack : Vec<char> = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            if c == '(' {
                stack.push('(');
            } else if c == '[' {
                stack.push('[');
            } else if c == '{' {
                stack.push('{');
            } else if c == '<' {
                stack.push('<');
            } else if c == ')' {
                if *(stack.last().unwrap()) == '(' {
                    stack.pop();
                } else {
                    score += 3;
                    corrupted = true;
                    break;
                }
            } else if c == ']' {
                if *(stack.last().unwrap()) == '[' {
                    stack.pop();
                } else {
                    score += 57;
                    corrupted = true;
                    break;
                }
            } else if c == '}' {
                if *(stack.last().unwrap()) == '{' {
                    stack.pop();
                } else {
                    score += 1197;
                    corrupted = true;
                    break;
                }
            } else if c == '>' {
                if *(stack.last().unwrap()) == '<' {
                    stack.pop();
                } else {
                    score += 25137;
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            incomplete_lines.push(line);
        }
    }
    println!("{}", score);
    return incomplete_lines;
}

fn part2(lines : &Vec<&str>) {
    let mut scores : Vec<i64> = Vec::new();
    for line in lines {
        let mut stack : Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '(' {
                stack.push('(');
            } else if c == '[' {
                stack.push('[');
            } else if c == '{' {
                stack.push('{');
            } else if c == '<' {
                stack.push('<');
            } else {
                stack.pop();
            }
        }
        let mut score : i64 = 0;
        while stack.len() > 0 {
            let c = stack.pop().unwrap();
            if c == '(' {
                score = (score * 5) + 1;
            } else if c == '[' {
                score = (score * 5) + 2;
            } else if c == '{' {
                score = (score * 5) + 3;
            } else if c == '<' {
                score = (score * 5) + 4;
            }
        }
        scores.push(score);
    }
    scores.sort();
    let mid = (scores.len() - 1) / 2;
    println!("{}", scores[mid]);
}
