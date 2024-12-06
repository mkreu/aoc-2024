use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Index, IndexMut},
};

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Clone, Debug)]
pub struct Input(Vec<Vec<char>>);

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    Input(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let start_row = input.0.iter().position(|row| row.contains(&'^')).unwrap();
    let start_col = input.0[start_row].iter().position(|c| *c == '^').unwrap();
    let mut pos = Vec2(start_row as isize, start_col as isize);
    let mut dir = Vec2(-1, 0);
    let mut input = input.clone();
    input[pos] = 'X';

    while !input.out_of_bounds(pos + dir) {
        if input[pos + dir] == '#' {
            dir = match dir {
                Vec2(-1, 0) => Vec2(0, 1),
                Vec2(0, 1) => Vec2(1, 0),
                Vec2(1, 0) => Vec2(0, -1),
                Vec2(0, -1) => Vec2(-1, 0),
                _ => panic!(),
            }
        } else {
            pos += dir;
            input[pos] = 'X';
        }

        //println!();
        //for line in &input.0 {
        //    println!("{}", line.iter().collect::<String>());
        //}
        //thread::sleep(Duration::from_millis(10));
    }
    input
        .0
        .iter()
        .map(|row| row.iter().filter(|c| **c == 'X').count())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let start_row = input.0.iter().position(|row| row.contains(&'^')).unwrap();
    let start_col = input.0[start_row].iter().position(|c| *c == '^').unwrap();
    let start = Vec2(start_row as isize, start_col as isize);
    let mut sum = 0;
    let size = input.0.len();
    for row in 0..size {
        for col in 0..size {
            let mut input = input.clone();
            let pos = Vec2(row as isize, col as isize);
            if pos == start {
                continue;
            }
            input[pos] = '#';
            if detect_loop(&input, start) {
                //println!("loop detected for: {:?}", pos);
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day6, part2, Mt)]
pub fn solve_part2_mt(input: &Input) -> usize {
    let start_row = input.0.iter().position(|row| row.contains(&'^')).unwrap();
    let start_col = input.0[start_row].iter().position(|c| *c == '^').unwrap();
    let start = Vec2(start_row as isize, start_col as isize);
    let size = input.0.len() as isize;
    (0..size)
        .map(|row| (0..size).map(move |col| Vec2(row, col)))
        .flatten()
        .par_bridge()
        .filter(|pos| {
            let mut input = input.clone();
            if *pos == start {
                return false;
            }
            input[*pos] = '#';
            detect_loop(&input, start)
        })
        .count()
}

fn detect_loop(input: &Input, start: Vec2) -> bool {
    let mut pos = start;
    let mut dir = Vec2(-1, 0);
    let mut history = HashSet::new();

    while !input.out_of_bounds(pos + dir) {
        if input[pos + dir] == '#' {
            dir = match dir {
                Vec2(-1, 0) => Vec2(0, 1),
                Vec2(0, 1) => Vec2(1, 0),
                Vec2(1, 0) => Vec2(0, -1),
                Vec2(0, -1) => Vec2(-1, 0),
                _ => panic!(),
            }
        } else {
            if !history.insert((pos, dir)) {
                return true;
            }
            pos += dir;
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(isize, isize);

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Index<Vec2> for Input {
    type Output = char;

    fn index(&self, index: Vec2) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<Vec2> for Input {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

impl Input {
    fn out_of_bounds(&self, index: Vec2) -> bool {
        index.0 < 0
            || index.0 >= self.0.len() as isize
            || index.1 < 0
            || index.1 >= self.0.len() as isize
    }
}
