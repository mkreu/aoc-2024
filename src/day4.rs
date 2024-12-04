use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input(Vec<Vec<char>>);

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    Input(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut buf = vec![];
    buf.extend(input.rows());
    buf.extend(input.columns().into_iter());
    buf.extend(input.diagonals().into_iter());
    buf.iter()
        .map(|line| {
            line.iter().collect::<String>().matches("XMAS").count()
                + line.iter().collect::<String>().matches("SAMX").count()
        })
        .sum()
}

impl Input {
    fn size(&self) -> usize {
        self.0.len()
    }
    fn rows(&self) -> impl Iterator<Item = Vec<char>> + use<'_> {
        self.0.iter().map(|row| row.clone())
    }
    fn columns(&self) -> impl Iterator<Item = Vec<char>> + use<'_> {
        (0..self.size()).map(|i| self.0.iter().map(|row| row[i]).collect())
    }

    fn diagonals(&self) -> Vec<Vec<char>> {
        let mut all = vec![];
        // left -> up
        for i in 0..self.size() {
            let mut buf = vec![];
            for j in 0..=i {
                buf.push(self.0[i - j][j]);
            }
            all.push(buf);
        }
        // left -> down
        for i in 0..self.size() {
            let mut buf = vec![];
            for j in i..self.size() {
                buf.push(self.0[j][j - i]);
            }
            all.push(buf);
        }

        // top -> right
        for i in 1..self.size() {
            let mut buf = vec![];
            for j in i..self.size() {
                buf.push(self.0[j - i][j]);
            }
            all.push(buf);
        }

        // bottom -> right
        for i in 1..self.size() {
            let mut buf = vec![];
            for j in i..self.size() {
                buf.push(self.0[self.size() - j + i - 1][j]);
            }
            all.push(buf);
        }
        all
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut count = 0;
    for i in 1..input.size() - 1 {
        for j in 1..input.size() - 1 {
            if is_x(&input.0, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn is_x(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    matrix[y][x] == 'A'
        && (matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M'
            || matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S')
        && (matrix[y - 1][x + 1] == 'S' && matrix[y + 1][x - 1] == 'M'
            || matrix[y - 1][x + 1] == 'M' && matrix[y + 1][x - 1] == 'S')
}
