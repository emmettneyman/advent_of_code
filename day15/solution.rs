use std::collections::BinaryHeap;
use std::collections::HashMap;

// Copied from 
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: (usize, usize),
    cost: usize,
}

fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = [[0; 100]; 100];
    for i in 0..100 {
        let row: Vec<u32> = lines[i]
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        for j in 0..100 {
            grid[i][j] = row[j] as usize;
        }
    }
    part1(grid);
}

fn part1(grid: [[usize; 100]; 100]) {
    let dist: HashMap<(usize, usize), u32> = HashMap::new();
    dist.insert((0, 0), 0);
    let heap = BinaryHeap::new();
}
