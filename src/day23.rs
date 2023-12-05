use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::Vector2;
use rustc_hash::FxHashSet;

pub type Pos = Vector2<i32>;

const CARD_DIRS: [Pos; 4] = [
    Pos::new(0, -1),
    Pos::new(0, 1),
    Pos::new(-1, 0),
    Pos::new(1, 0),
];
const NEIGHBORS: [Pos; 8] = [
    Pos::new(-1, -1),
    Pos::new(0, -1),
    Pos::new(1, -1),
    Pos::new(-1, 0),
    Pos::new(1, 0),
    Pos::new(-1, 1),
    Pos::new(0, 1),
    Pos::new(1, 1),
];
const CLEARANCE: [[Pos; 3]; 4] = [
    [Pos::new(-1, -1), Pos::new(0, -1), Pos::new(1, -1)],
    [Pos::new(-1, 1), Pos::new(0, 1), Pos::new(1, 1)],
    [Pos::new(-1, -1), Pos::new(-1, 0), Pos::new(-1, 1)],
    [Pos::new(1, -1), Pos::new(1, 0), Pos::new(1, 1)],
];

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> FxHashSet<Pos> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Pos::new(x as i32, y as i32))
        })
        .collect()
}

fn step<const MAX_STEPS: usize>(grid: &FxHashSet<Pos>) -> (usize, FxHashSet<Pos>) {
    fn step_once(grid: &mut FxHashSet<Pos>, i: usize) -> bool {
        let mut moves = 0usize;
        let old_grid = grid.clone();
        grid.clear();

        'outer: for pos in &old_grid {
            if NEIGHBORS.iter().any(|p| old_grid.contains(&(pos + p))) {
                for orig_dir_index in 0..4 {
                    let dir_index = (orig_dir_index + i) % 4;
                    if CLEARANCE[dir_index]
                        .iter()
                        .all(|p| !old_grid.contains(&(pos + p)))
                    {
                        let dir = &CARD_DIRS[dir_index];
                        let target = pos + dir;
                        if !grid.insert(target) {
                            grid.remove(&target);
                            grid.insert(*pos);
                            grid.insert(target + dir);
                            moves -= 2;
                        } else {
                            moves += 1;
                        }

                        continue 'outer;
                    }
                }
            }

            grid.insert(*pos);
        }

        moves > 0
    }

    let mut grid = grid.clone();
    let mut i = 0;
    loop {
        if i >= MAX_STEPS {
            break;
        }

        i += 1;
        if !step_once(&mut grid, i - 1) {
            break;
        }
    }

    (i, grid)
}

fn open_spaces(grid: &FxHashSet<Pos>) -> usize {
    let min_x = grid.iter().map(|p| p.x).min().unwrap();
    let max_x = grid.iter().map(|p| p.x).max().unwrap();
    let min_y = grid.iter().map(|p| p.y).min().unwrap();
    let max_y = grid.iter().map(|p| p.y).max().unwrap();
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - grid.len()
}

#[aoc(day23, part1)]
pub fn part1(input: &FxHashSet<Pos>) -> usize {
    let (_, grid) = step::<10>(input);
    open_spaces(&grid)
}

#[aoc(day23, part2)]
pub fn part2(input: &FxHashSet<Pos>) -> usize {
    let (n, _) = step::<{ usize::MAX }>(input);
    n
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_small() {
        let input = input_generator(
            r".....
..##.
..#..
.....
..##.
.....
",
        );
        assert_eq!(25, part1(&input))
    }

    #[test]
    fn test_1() {
        let input = input_generator(
            r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
",
        );
        assert_eq!(110, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(
            r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
",
        );
        assert_eq!(20, part2(&input))
    }
}
