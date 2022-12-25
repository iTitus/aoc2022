use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::Vector2;
use pathfinding::prelude::*;
use rustc_hash::FxHashSet;

pub type Pos = Vector2<i32>;

const NORTH: Pos = Pos::new(0, -1);
const EAST: Pos = Pos::new(1, 0);
const SOUTH: Pos = Pos::new(0, 1);
const WEST: Pos = Pos::new(-1, 0);
const DIRS: [Pos; 5] = [Pos::new(0, 0), NORTH, EAST, SOUTH, WEST];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn vec(&self) -> &'static Pos {
        match self {
            Dir::North => &NORTH,
            Dir::East => &EAST,
            Dir::South => &SOUTH,
            Dir::West => &WEST,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Blizzard {
    dir: Dir,
    pos: Pos,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Valley {
    blizzards: Vec<Blizzard>,
    size_x: i32,
    size_y: i32,
}

impl Valley {
    pub fn get_blizzard_state(&self, steps: i32) -> FxHashSet<Pos> {
        self.blizzards.iter()
            .map(|b| {
                let mut pos = b.pos + steps * b.dir.vec();
                pos.x = (pos.x - 1).rem_euclid(self.size_x - 2) + 1;
                pos.y = (pos.y - 1).rem_euclid(self.size_y - 2) + 1;
                pos
            })
            .collect()
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Valley {
    let mut size_x = 0;
    let mut size_y = 0;
    let blizzards = input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, l)| {
            size_y = size_y.max(y as i32 + 1);
            size_x = l.len() as i32;
            l.chars()
                .enumerate()
                .filter(|(_, c)| matches!(c, '^' | '>' | 'v' | '<'))
                .map(move |(x, c)| {
                    let dir = match c {
                        '^' => Dir::North,
                        '>' => Dir::East,
                        'v' => Dir::South,
                        '<' => Dir::West,
                        _ => unreachable!()
                    };
                    Blizzard {
                        dir,
                        pos: Pos::new(x as i32, y as i32),
                    }
                })
        })
        .collect();
    assert!(size_x >= 3);
    assert!(size_y >= 3);
    Valley {
        blizzards,
        size_x,
        size_y,
    }
}

fn do_pathfinding(valley: &Valley, start_minutes: i32, start_pos: Pos, end_pos: Pos) -> i32 {
    let (path, _cost) = astar(
        &(start_minutes, start_pos),
        |&(i, pos)| {
            let blizzards = valley.get_blizzard_state(i + 1);
            DIRS.iter()
                .map(|&dir| {
                    let pos: Pos = pos + dir;
                    pos
                })
                .filter(|&pos| pos == end_pos || pos == start_pos || (pos.x >= 1 && pos.x <= valley.size_x - 2 && pos.y >= 1 && pos.y <= valley.size_y - 2))
                .filter(|pos| !blizzards.contains(pos))
                .map(|pos| ((i + 1, pos), 1))
                .collect_vec()
        },
        |&(_, pos)| (end_pos - pos).abs().sum(),
        |&(_, pos)| pos == end_pos,
    ).unwrap();
    path.last().unwrap().0
}

#[aoc(day24, part1)]
pub fn part1(valley: &Valley) -> i32 {
    let start = Pos::new(1, 0);
    let goal = Pos::new(valley.size_x - 2, valley.size_y - 1);
    do_pathfinding(valley, 0, start, goal)
}

#[aoc(day24, part2)]
pub fn part2(valley: &Valley) -> i32 {
    let start = Pos::new(1, 0);
    let goal = Pos::new(valley.size_x - 2, valley.size_y - 1);
    let start_to_goal = do_pathfinding(valley, 0, start, goal);
    let goal_to_start = do_pathfinding(valley, start_to_goal, goal, start);
    do_pathfinding(valley, goal_to_start, start, goal)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
");
        assert_eq!(18, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
");
        assert_eq!(54, part2(&input))
    }
}
