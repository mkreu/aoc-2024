use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<char>>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut antennas: HashMap<char, Vec<Vec2>> = HashMap::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == '.' {
                continue;
            }
            antennas
                .entry(input[row][col])
                .or_default()
                .push(Vec2(row as isize, col as isize));
        }
    }
    let mut antinodes: HashSet<Vec2> = HashSet::new();
    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let ij = positions[j] - positions[i];
                antinodes.insert(positions[j] + ij);
                antinodes.insert(positions[i] - ij);
            }
        }
    }
    antinodes
        .into_iter()
        .filter(|pos| {
            pos.0 >= 0
                && (pos.0 as usize) < input.len()
                && pos.1 >= 0
                && (pos.1 as usize) < input.len()
        })
        .count()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut antennas: HashMap<char, Vec<Vec2>> = HashMap::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == '.' {
                continue;
            }
            antennas
                .entry(input[row][col])
                .or_default()
                .push(Vec2(row as isize, col as isize));
        }
    }
    let mut antinodes: HashSet<Vec2> = HashSet::new();
    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let mut pi = positions[i];
                let mut pj = positions[j];
                let ij = pj - pi;
                while in_bounds(pi, input.len()) {
                    antinodes.insert(pi);
                    pi = pi - ij;
                }
                while in_bounds(pj, input.len()) {
                    antinodes.insert(pj);
                    pj = pj + ij;
                }
            }
        }
    }
    antinodes.into_iter().count()
}

fn in_bounds(pos: Vec2, size: usize) -> bool {
    pos.0 >= 0 && (pos.0 as usize) < size && pos.1 >= 0 && (pos.1 as usize) < size
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(isize, isize);

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}
