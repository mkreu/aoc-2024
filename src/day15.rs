use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, Rem};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    map: Map,
    instructions: Vec<Vec2>,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let map = Map(split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect());
    let instructions = split
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '>' => Vec2(0, 1),
            '<' => Vec2(0, -1),
            '^' => Vec2(-1, 0),
            'v' => Vec2(1, 0),
            _ => panic!("invalid instruction: {}", c),
        })
        .collect();
    Input { map, instructions }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut pos = robot_pos(&input.map);
    let mut map = input.map.clone();

    for inst in &input.instructions {
        if rec1(&mut map, pos, *inst) {
            pos += *inst;
        }
        //println!("{:?}", inst);
        //map.iter().for_each(|row| {
        //    println!("{}", row.iter().collect::<String>());
        //});
    }

    gps(&map)
}

fn rec1(map: &mut Map, pos: Vec2, dir: Vec2) -> bool {
    match map[pos] {
        'O' | '@' => {
            if rec1(map, pos + dir, dir) {
                map[pos + dir] = map[pos];
                map[pos] = '.';
                true
            } else {
                false
            }
        }
        '.' => true,
        '#' => false,
        _ => panic!(),
    }
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut map = Map(input
        .map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| match c {
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    '.' => vec!['.', '.'],
                    '#' => vec!['#', '#'],
                    _ => vec![*c],
                })
                .collect()
        })
        .collect());
    let mut pos = robot_pos(&map);

    for inst in &input.instructions {
        if rec2(&mut map, pos, *inst) {
            pos += *inst;
        }
        //println!("{:?}", inst);
        //map.iter().for_each(|row| {
        //    println!("{}", row.iter().collect::<String>());
        //});
    }

    map.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });

    gps(&map)
}

fn rec2(map: &mut Map, pos: Vec2, dir: Vec2) -> bool {
    match map[pos] {
        '@' => {
            if rec2(map, pos + dir, dir) {
                map[pos + dir] = map[pos];
                map[pos] = '.';
                true
            } else {
                false
            }
        }
        '[' | ']' => {
            if dir.0 == 0 {
                if rec2(map, pos + dir, dir) {
                    map[pos + dir] = map[pos];
                    map[pos] = '.';
                    true
                } else {
                    false
                }
            } else if map[pos] == map[pos + dir] {
                let pos2 = pos + Vec2(0, if map[pos] == '[' { 1 } else { -1 });
                if rec2(map, pos + dir, dir) {
                    map[pos + dir] = map[pos];
                    map[pos2 + dir] = map[pos2];
                    map[pos] = '.';
                    map[pos2] = '.';
                    true
                } else {
                    false
                }
            } else {
                let pos2 = pos + Vec2(0, if map[pos] == '[' { 1 } else { -1 });
                if rec2(map, pos + dir, dir) && rec2(map, pos2 + dir, dir) {
                    map[pos + dir] = map[pos];
                    map[pos2 + dir] = map[pos2];
                    map[pos] = '.';
                    map[pos2] = '.';
                    true
                } else {
                    false
                }
            }
        }
        '.' => true,
        '#' => false,
        _ => panic!(),
    }
}

fn robot_pos(map: &Map) -> Vec2 {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| {
                if c == '@' {
                    Some(Vec2(y as isize, x as isize))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn gps(map: &Map) -> usize {
    let mut sum = 0;
    for row in 0..map.len() {
        for col in 0..map.0[row].len() {
            if map.0[row][col] == 'O' || map.0[row][col] == '[' {
                sum += row * 100 + col;
            }
        }
    }
    sum
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<char>>);

impl Deref for Map {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<Vec2> for Map {
    type Output = char;

    fn index(&self, index: Vec2) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<Vec2> for Map {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(isize, isize);

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 += rhs.0;
        self.1 += rhs.1;
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
