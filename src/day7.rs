use aoc_runner_derive::{aoc, aoc_generator};

pub struct Line {
    test: usize,
    values: Vec<usize>,
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let test = split.next().unwrap().parse().unwrap();
            let values = split
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            Line { test, values }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Line>) -> usize {
    input
        .into_iter()
        .filter(|rec| {
            op_mask1(rec.values.len() - 1).into_iter().any(|mask| {
                rec.values
                    .iter()
                    .skip(1)
                    .zip(mask)
                    .fold(rec.values[0], |acc, (val, op)| match op {
                        Op1::Add => acc + val,
                        Op1::Mul => acc * val,
                    })
                    == rec.test
            })
        })
        .map(|rec| rec.test)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Line>) -> usize {
    input
        .into_iter()
        .filter(|rec| {
            op_mask2(rec.values.len() - 1).into_iter().any(|mask| {
                rec.values
                    .iter()
                    .skip(1)
                    .zip(mask)
                    .fold(rec.values[0], |acc, (val, op)| match op {
                        Op2::Add => acc + val,
                        Op2::Mul => acc * val,
                        Op2::Concat => (acc.to_string() + val.to_string().as_str())
                            .parse()
                            .unwrap(),
                    })
                    == rec.test
            })
        })
        .map(|rec| rec.test)
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Op1 {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum Op2 {
    Add,
    Mul,
    Concat,
}

fn op_mask1(len: usize) -> Vec<Vec<Op1>> {
    if len == 0 {
        vec![vec![]]
    } else {
        let mut res: Vec<Vec<Op1>> = Vec::new();
        res.extend(op_mask1(len - 1).into_iter().map(|mut vec| {
            vec.push(Op1::Add);
            vec
        }));
        res.extend(op_mask1(len - 1).into_iter().map(|mut vec| {
            vec.push(Op1::Mul);
            vec
        }));
        res
    }
}

fn op_mask2(len: usize) -> Vec<Vec<Op2>> {
    if len == 0 {
        vec![vec![]]
    } else {
        let mut res: Vec<Vec<Op2>> = Vec::new();
        res.extend(op_mask2(len - 1).into_iter().map(|mut vec| {
            vec.push(Op2::Add);
            vec
        }));
        res.extend(op_mask2(len - 1).into_iter().map(|mut vec| {
            vec.push(Op2::Mul);
            vec
        }));
        res.extend(op_mask2(len - 1).into_iter().map(|mut vec| {
            vec.push(Op2::Concat);
            vec
        }));
        res
    }
}
