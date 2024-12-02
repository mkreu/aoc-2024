use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let (left, right) = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .fold((vec![], vec![]), |(mut left, mut right), v| {
            left.push(v[0].parse::<u32>().unwrap());
            right.push(v[1].parse::<u32>().unwrap());
            (left, right)
        });
    Input { left, right }
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let (mut left, mut right) = (input.left.clone(), input.right.clone());
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input
        .left
        .iter()
        .map(|l| l * input.right.iter().filter(|r| **r == *l).count() as u32)
        .sum()
}
