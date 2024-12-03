use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<usize>().unwrap();
            let b = cap[2].parse::<usize>().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .fold((true, vec![]), |(doo, mut acc), cap| {
            if cap[0] == *"do()" {
                (true, acc)
            } else if cap[0] == *"don't()" {
                (false, acc)
            } else if doo {
                let a = cap[1].parse::<usize>().unwrap();
                let b = cap[2].parse::<usize>().unwrap();
                acc.push(a * b);
                (doo, acc)
            } else {
                (doo, acc)
            }
        })
        .1
        .iter()
        .sum()
}
