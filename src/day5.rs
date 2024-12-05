use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Input {
    after: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let updates = split.next().unwrap();
    let updates = updates
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();
    let mut after: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in rules.lines() {
        let mut split = line.split("|");
        let left = split.next().unwrap().parse().unwrap();
        let right = split.next().unwrap().parse().unwrap();
        after.entry(left).or_default().push(right);
    }
    Input { after, updates }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> usize {
    println!("{:?}", input);
    input
        .updates
        .iter()
        .filter(|update| is_valid(input, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> usize {
    println!("{:?}", input);
    input
        .updates
        .iter()
        .filter(|update| !is_valid(input, update))
        .cloned()
        .map(|mut update| {
            update.sort_by(|a, b| {
                if input.after.get(a).map(|v| v.contains(b)).unwrap_or(false) {
                    std::cmp::Ordering::Less
                } else if input.after.get(b).map(|v| v.contains(a)).unwrap_or(false) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_valid(input: &Input, update: &[usize]) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if !input
                .after
                .get(&update[i])
                .map(|v| v.contains(&update[j]))
                .unwrap_or(true)
            {
                return false;
            }
            if input
                .after
                .get(&update[j])
                .map(|v| v.contains(&update[i]))
                .unwrap_or(false)
            {
                return false;
            }
        }
    }
    true
}
