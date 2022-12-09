use std::collections::HashSet;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

type Num = i32;

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Down,
    Up,
    Left,
    Right,
}

impl Dir {
    fn get_vector(&self) -> (Num, Num) {
        match self {
            Dir::Down => (0, -1),
            Dir::Up => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn move_vec(&self, (x, y): (Num, Num)) -> (Num, Num) {
        let (dx, dy) = self.get_vector();
        (x + dx, y + dy)
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Dir::Down),
            "U" => Ok(Dir::Up),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(())
        }
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(Dir, usize)> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (dir, amount) = l.split_once(' ').unwrap();
            (dir.parse().unwrap(), amount.parse().unwrap())
        })
        .collect()
}

fn follow_head(knots: &mut [(Num, Num)]) {
    for i in 1..knots.len() {
        let (hx, hy) = &knots[i - 1];
        let (x, y) = &knots[i];
        if hx.abs_diff(*x) > 1 || hy.abs_diff(*y) > 1 {
            knots[i] = (x + (hx - x).signum(), y + (hy - y).signum());
        } else {
            break;
        }
    }
}

fn simulate_rope<const KNOTS: usize>(moves: &[(Dir, usize)]) -> usize {
    assert!(KNOTS > 0);
    let mut knots = [(0, 0); KNOTS];
    let mut all_tail_pos = HashSet::new();
    all_tail_pos.insert(knots[KNOTS - 1]);
    for (dir, amount) in moves {
        for _ in 0..*amount {
            knots[0] = dir.move_vec(knots[0]);
            follow_head(&mut knots);
            all_tail_pos.insert(knots[KNOTS - 1]);
        }
    }

    all_tail_pos.len()
}

#[aoc(day9, part1)]
pub fn part1(input: &[(Dir, usize)]) -> usize {
    simulate_rope::<2>(input)
}

#[aoc(day9, part2)]
pub fn part2(input: &[(Dir, usize)]) -> usize {
    simulate_rope::<10>(input)
}
