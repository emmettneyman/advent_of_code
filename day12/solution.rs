use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split("-").collect();
        let v1 = split[0];
        let v2 = split[1];
        if !graph.contains_key(v1) {
            graph.insert(v1, vec![v2]);
        } else {
            let mut v = graph.get(v1).unwrap().clone();
            v.push(v2);
            graph.insert(v1, v);
        }
        if !graph.contains_key(v2) {
            graph.insert(v2, vec![v1]);
        } else {
            let mut v = graph.get(v2).unwrap().clone();
            v.push(v1);
            graph.insert(v2, v);
        }
    }
    part1(&graph);
    part2(&graph);
}

fn part1(graph: &HashMap<&str, Vec<&str>>) {
    let mut q: VecDeque<Vec<&str>> = VecDeque::new();
    q.push_back(vec!["start"]);
    let mut num_paths = 0;
    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        for neighbor in graph.get(curr.last().unwrap()).unwrap() {
            if *neighbor == "end" {
                num_paths += 1;
            } else if (neighbor.to_uppercase() == *neighbor) || !curr.contains(neighbor) {
                let mut new_path = curr.clone();
                new_path.push(neighbor);
                q.push_back(new_path);
            }
        }
    }
    println!("{}", num_paths);
}

fn part2(graph: &HashMap<&str, Vec<&str>>) {
    let mut q: VecDeque<VecDeque<&str>> = VecDeque::new();
    q.push_back(VecDeque::from(["start"]));
    let mut num_paths = 0;
    while q.len() > 0 {
        let curr = q.pop_front().unwrap();
        for neighbor in graph.get(curr.back().unwrap()).unwrap() {
            let should_repeat = *neighbor == neighbor.to_lowercase() && curr.contains(neighbor);
            if *neighbor == "end" {
                num_paths += 1;
            } else if *neighbor != "start" && !(curr[0] == "rep" && should_repeat) {
                let mut new_path = curr.clone();
                if should_repeat {
                    new_path.push_front("rep");
                }
                new_path.push_back(neighbor);
                q.push_back(new_path);
            }
        }
    }
    println!("{}", num_paths);
}
