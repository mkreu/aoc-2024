use std::{
    fmt::Debug,
    iter,
    ops::{Deref, DerefMut},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Fs {
    let compact: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let extended: Vec<Option<u32>> = compact
        .iter()
        .enumerate()
        .flat_map(|(i, f)| {
            iter::repeat_n(
                if i % 2 == 0 { Some(i as u32 / 2) } else { None },
                *f as usize,
            )
        })
        .collect();
    Fs(extended)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Fs) -> usize {
    let mut fs = input.clone();
    println!("{:?}", fs);

    let (mut l, mut r) = (0, fs.len() - 1);
    while l <= r {
        if fs[l].is_some() {
            l += 1;
        } else {
            fs.swap(l, r);
            r -= 1;
        }
    }
    println!("{:?}", fs);
    fs.iter()
        .enumerate()
        .map(|(i, f)| i * (f.unwrap_or(0) as usize))
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Fs) -> usize {
    let mut fs = input.clone();
    println!("{:?}", fs);

    let mut len = 1;
    for r in (1..fs.len()).rev() {
        if fs[r] != fs[r - 1] {
            if len > 0 {
                for l in 0..fs.len() {
                    if l >= r {
                        break;
                    }
                    if fs[l..l + len].iter().all(|o| o.is_none()) {
                        for i in 0..len {
                            fs.swap(l + i, r + i);
                        }
                        break;
                    }
                }
            }
            len = 1;
        } else {
            len += 1;
        }
    }

    println!("{:?}", fs);
    fs.iter()
        .enumerate()
        .map(|(i, f)| i * (f.unwrap_or(0) as usize))
        .sum()
}

#[derive(Clone)]
pub struct Fs(Vec<Option<u32>>);

impl Debug for Fs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.0
                .iter()
                .map(|f| f.map(|i| i.to_string()).unwrap_or(".".to_string()))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl DerefMut for Fs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Fs {
    type Target = Vec<Option<u32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
