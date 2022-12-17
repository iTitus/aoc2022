use std::collections::hash_map::Entry;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const BLOCKS: [[(u64, u64); 7]; 5] = [
    [(0, 0), (0, 0), (0, 1), (0, 1), (0, 1), (0, 1), (0, 0)],
    [(0, 0), (0, 0), (1, 1), (0, 3), (1, 1), (0, 0), (0, 0)],
    [(0, 0), (0, 0), (0, 1), (0, 1), (0, 3), (0, 0), (0, 0)],
    [(0, 0), (0, 0), (0, 4), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (0, 0), (0, 2), (0, 2), (0, 0), (0, 0), (0, 0)],
];

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}

impl TryFrom<u8> for Dir {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Dir::Left),
            b'>' => Ok(Dir::Right),
            _ => Err(())
        }
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Dir> {
    input.trim().bytes()
        .map(|b| Dir::try_from(b).unwrap())
        .collect()
}

const WIDTH: usize = 7;

fn calculate_normalized_surface(levels: &[FxHashSet<u64>]) -> Vec<(u64, u64)> {
    let mut all_pos = FxHashSet::default();
    let mut pos = (0u64, *levels[0].iter().max().unwrap());
    all_pos.insert(pos);

    let final_pos = ((WIDTH as u64) - 1, *levels[WIDTH - 1].iter().max().unwrap());

    let moves = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
    let mut index = 0;
    'outer: while pos != final_pos {
        loop {
            let next_move = &moves[index];
            let new_x = (pos.0 as i64) + next_move.0;
            let new_y = (pos.1 as i64) + next_move.1;
            if (0..(WIDTH as i64)).contains(&new_x) && new_y >= 0 && levels[new_x as usize].contains(&(new_y as u64)) {
                pos.0 = new_x as u64;
                pos.1 = new_y as u64;
                all_pos.insert(pos);
                index = (index + 1 + (moves.len() / 2)) % moves.len();
                continue 'outer;
            }

            index = (index + 1) % moves.len();
        }
    }

    let min_y = *all_pos.iter().map(|(_, y)| y).min().unwrap();
    all_pos.into_iter().map(|(x, y)| (x, y - min_y)).collect()
}

fn tetris<const N: usize>(jet_dirs: &[Dir]) -> u64 {
    let mut cache = FxHashMap::default();

    let mut levels: Vec<FxHashSet<u64>> = vec![FxHashSet::default(); WIDTH];
    levels.iter_mut().for_each(|x| { x.insert(0); });

    let mut saved_y_growth = 0;
    let mut jet_index = 0;
    let mut i = 0;
    loop {
        let block_index = i % BLOCKS.len();

        let max_y = *levels.iter().flatten().max().unwrap();
        let cache_key = (block_index, jet_index, calculate_normalized_surface(&levels));
        let entry = cache.entry(cache_key);
        match entry {
            Entry::Occupied(e) => {
                let (first_i, first_max_y) = e.get();
                let cycle_length = i - first_i;
                let y_growth = max_y - first_max_y;
                let periods = (N - i) / cycle_length;
                i += periods * cycle_length;
                saved_y_growth += (periods as u64) * y_growth;
            }
            Entry::Vacant(e) => { e.insert((i, max_y)); }
        }

        let block = &BLOCKS[block_index];
        let _print_levels = levels.iter().map(|x| x.iter().max().unwrap()).collect_vec();

        let mut block_y = max_y + 4;
        let mut offset_x = 0isize;
        'fall: loop {
            let _d = jet_dirs[jet_index];
            let horizontal_check = |dx| {
                for bx in 0..WIDTH {
                    let x = bx as isize + (offset_x + dx);
                    if x < 0 || x as usize >= WIDTH {
                        if block[bx].1 != 0 {
                            return 0;
                        }

                        continue;
                    }
                    let x = x as usize;
                    let (y_start, height) = block[bx];
                    if height > 0 && ((y_start + block_y)..(y_start + block_y + height)).any(|y| levels[x].contains(&y)) {
                        return 0;
                    }
                }

                dx
            };

            offset_x += match jet_dirs[jet_index] {
                Dir::Left => horizontal_check(-1),
                Dir::Right => horizontal_check(1)
            };

            jet_index = (jet_index + 1) % jet_dirs.len();

            for bx in 0..WIDTH {
                let x = bx as isize + offset_x;
                if x < 0 || x as usize >= WIDTH {
                    assert_eq!(block[bx].1, 0);
                    continue;
                }
                let x = x as usize;

                let (y_start, height) = block[bx];
                let current_y = y_start + block_y;
                if height > 0 && levels[x].contains(&(current_y - 1)) {
                    for x in 0..WIDTH {
                        let bx = x as isize - offset_x;
                        if bx < 0 || bx as usize >= WIDTH {
                            continue;
                        }
                        let bx = bx as usize;

                        let (y_start, height) = block[bx];
                        if height > 0 {
                            levels[x].extend((y_start + block_y)..(y_start + block_y + height));
                        }
                    }

                    break 'fall;
                }
            }

            block_y -= 1;
        }

        i += 1;
        if i >= N {
            break;
        }
    }

    saved_y_growth + *levels.iter().flatten().max().unwrap()
}

#[aoc(day17 part1)]
pub fn part1(jet_dirs: &[Dir]) -> u64 {
    tetris::<2022>(jet_dirs)
}

#[aoc(day17, part2)]
pub fn part2(jet_dirs: &[Dir]) -> u64 {
    tetris::<1000000000000>(jet_dirs)
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
");
        assert_eq!(3068, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
");
        assert_eq!(1514285714288, part2(&input))
    }
}
