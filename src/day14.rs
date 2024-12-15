use std::ops::{Add, Mul, Rem};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub type Input = Vec<Robot>;

const WIDTH: isize = 101;
const HEIGTH: isize = 103;
//const WIDTH: isize = 11;
//const HEIGTH: isize = 7;

#[derive(Debug, Clone)]
pub struct Robot {
    pos: Vec2,
    dir: Vec2,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=([-0-9]+),([-0-9]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Robot {
                pos: Vec2(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                dir: Vec2(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let end_positions: Vec<Vec2> = input
        .iter()
        .map(|robot| (robot.pos + (robot.dir * 100)) % SIZE)
        .collect();
    let lu = end_positions
        .iter()
        .filter(|pos| pos.0 < WIDTH / 2 && pos.1 < HEIGTH / 2)
        .count();
    let ru = end_positions
        .iter()
        .filter(|pos| pos.0 > WIDTH / 2 && pos.1 < HEIGTH / 2)
        .count();
    let lb = end_positions
        .iter()
        .filter(|pos| pos.0 < WIDTH / 2 && pos.1 > HEIGTH / 2)
        .count();
    let rb = end_positions
        .iter()
        .filter(|pos| pos.0 > WIDTH / 2 && pos.1 > HEIGTH / 2)
        .count();
    dbg!(lu, ru, lb, rb);
    lu * ru * lb * rb
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut robots = input.clone();
    let mut seconds = 0;
    loop {
        let mut map = vec![vec![' '; WIDTH as usize]; HEIGTH as usize];
        for robot in robots.iter() {
            map[robot.pos.1 as usize][robot.pos.0 as usize] = '#';
        }
        let x_dev =
            std_deviation(&robots.iter().map(|robot| robot.pos.0).collect::<Vec<_>>()).unwrap();
        let y_dev =
            std_deviation(&robots.iter().map(|robot| robot.pos.1).collect::<Vec<_>>()).unwrap();
        if x_dev < 25.0 && y_dev < 25.0 {
            println!("Seconds: {}", seconds);
            println!("{}, {}", x_dev, y_dev);
            for row in map.iter() {
                println!("{}", row.iter().collect::<String>());
            }
            return seconds;
        }
        for robot in robots.iter_mut() {
            robot.pos = (robot.pos + robot.dir) % SIZE;
        }
        seconds += 1;
    }
}

fn mean(data: &[isize]) -> Option<f32> {
    let sum = data.iter().sum::<isize>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[isize]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

const SIZE: Vec2 = Vec2(WIDTH, HEIGTH);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(isize, isize);

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: isize) -> Vec2 {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl Rem<Vec2> for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}
