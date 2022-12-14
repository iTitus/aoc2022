use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub enum Obstacle {
    LineX { y: u32, x_start: u32, x_end: u32 },
    LineY { x: u32, y_start: u32, y_end: u32 },
    Sand(Pos),
}

impl Obstacle {
    pub fn contains(&self, pos: &Pos) -> bool {
        match self {
            Obstacle::LineX { y, x_start, x_end } => { y == &pos.y && (x_start..=x_end).contains(&&pos.x) }
            Obstacle::LineY { x, y_start, y_end } => { x == &pos.x && (y_start..=y_end).contains(&&pos.y) }
            Obstacle::Sand(p) => p == pos
        }
    }

    pub fn max_y(&self) -> u32 {
        match self {
            Obstacle::LineX { y, .. } => *y,
            Obstacle::LineY { y_end, .. } => *y_end,
            Obstacle::Sand(Pos { y, .. }) => *y
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    x: u32,
    y: u32,
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        Ok(Pos { x: x.parse().map_err(|_| ())?, y: y.parse().map_err(|_| ())? })
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Obstacle> {
    let paths: Vec<Vec<Pos>> = input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.split("->")
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|p| p.parse().unwrap())
            .collect()
        )
        .collect();

    let mut obstacles = Vec::new();
    for path in paths {
        for window in path.windows(2) {
            if let [a, b] = window {
                if a.y == b.y {
                    obstacles.push(Obstacle::LineX { y: a.y, x_start: a.x.min(b.x), x_end: a.x.max(b.x) });
                } else if a.x == b.x {
                    obstacles.push(Obstacle::LineY { x: a.x, y_start: a.y.min(b.y), y_end: a.y.max(b.y) });
                } else {
                    panic!("diagonal lines not allowed");
                }
            }
        }
    }

    obstacles
}

const SPAWN_POINT: Pos = Pos { x: 500, y: 0 };

#[aoc(day14 part1)]
pub fn part1(obstacles: &[Obstacle]) -> usize {
    let mut obstacles = obstacles.to_vec();
    let max_y = obstacles.iter().map(|o| o.max_y()).max().unwrap();

    let mut count = 0;
    'outer: loop {
        let mut sand = SPAWN_POINT;
        if obstacles.iter().any(|o| o.contains(&sand)) {
            panic!();
        }

        'inner: loop {
            for next_pos in [Pos { x: sand.x, y: sand.y + 1 }, Pos { x: sand.x - 1, y: sand.y + 1 }, Pos { x: sand.x + 1, y: sand.y + 1 }] {
                if obstacles.iter().any(|o| o.contains(&next_pos)) {
                    continue;
                }

                if sand.y > max_y {
                    break 'outer;
                }

                sand = next_pos;
                continue 'inner;
            }

            count += 1;
            obstacles.push(Obstacle::Sand(sand));
            continue 'outer;
        }
    }

    count
}

#[aoc(day14, part2)]
pub fn part2(obstacles: &[Obstacle]) -> usize {
    let mut obstacles = obstacles.to_vec();
    let floor = 2 + obstacles.iter().map(|o| o.max_y()).max().unwrap();

    let mut count = 0;
    'outer: loop {
        let mut sand = SPAWN_POINT;
        if obstacles.iter().any(|o| o.contains(&sand)) {
            break 'outer;
        }

        'inner: loop {
            for next_pos in [Pos { x: sand.x, y: sand.y + 1 }, Pos { x: sand.x - 1, y: sand.y + 1 }, Pos { x: sand.x + 1, y: sand.y + 1 }] {
                if next_pos.y >= floor || obstacles.iter().any(|o| o.contains(&next_pos)) {
                    continue;
                }

                sand = next_pos;
                continue 'inner;
            }

            count += 1;
            obstacles.push(Obstacle::Sand(sand));
            continue 'outer;
        }
    }

    count
}
