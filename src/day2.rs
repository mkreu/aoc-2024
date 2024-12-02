use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input(Vec<Vec<usize>>);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    Input(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input.0.iter().filter(|row| is_safe(&row)).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Input) -> usize {
    input
        .0
        .iter()
        .filter(|row| {
            iter::repeat_n(row, row.len())
                .enumerate()
                .map(|(i, rec)| {
                    rec.iter()
                        .enumerate()
                        .filter(|(j, _)| i != *j)
                        .map(|(_, n)| *n)
                        .collect::<Vec<_>>()
                })
                .any(|rec| is_safe(&rec))
        })
        .count()
}

fn is_safe(report: &[usize]) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted())
        && report
            .windows(2)
            .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
}
