use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use tinyvec::array_vec;

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
    start: usize,
    end: usize,
}

impl Grid {
    pub fn new(mut trees: Vec<u8>, width: usize) -> Grid {
        let height = trees.len() / width;
        if trees.len() != width * height {
            panic!()
        }

        let (start, _) = trees.iter().find_position(|&&n| n == b'S').unwrap();
        trees[start] = b'a';

        let (end, _) = trees.iter().find_position(|&&n| n == b'E').unwrap();
        trees[end] = b'z';

        Grid { grid: trees, width, height, start, end }
    }

    fn to_idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn to_coords(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn bfs_impl(&self, source: usize, can_step_to: impl Fn(u8, u8) -> bool) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.grid.len()];
        dist[source] = 0;

        let mut q = VecDeque::new();
        q.push_back(source);

        while let Some(n) = q.pop_front() {
            let value = self.grid[n];

            let mut neighbors = array_vec!([usize; 4]);
            let (x, y) = self.to_coords(n);
            if x > 0 {
                let idx = self.to_idx(x - 1, y);
                if can_step_to(value, self.grid[idx]) {
                    neighbors.push(idx);
                }
            }
            if x + 1 < self.width {
                let idx = self.to_idx(x + 1, y);
                if can_step_to(value, self.grid[idx]) {
                    neighbors.push(idx);
                }
            }
            if y > 0 {
                let idx = self.to_idx(x, y - 1);
                if can_step_to(value, self.grid[idx]) {
                    neighbors.push(idx);
                }
            }
            if y + 1 < self.height {
                let idx = self.to_idx(x, y + 1);
                if can_step_to(value, self.grid[idx]) {
                    neighbors.push(idx);
                }
            }

            let from_dist = dist[n];
            neighbors.into_iter()
                .for_each(|to| {
                    let new_dist = from_dist + 1;
                    if new_dist < dist[to] {
                        dist[to] = new_dist;
                        q.push_back(to);
                    }
                });
        }

        dist
    }

    pub fn bfs(&self) -> Vec<usize> {
        fn can_step_to(from: u8, to: u8) -> bool {
            to <= from + 1
        }

        self.bfs_impl(self.start, can_step_to)
    }

    pub fn bfs_reverse(&self) -> Vec<usize> {
        fn can_step_to(from: u8, to: u8) -> bool {
            from <= to + 1
        }

        self.bfs_impl(self.end, can_step_to)
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Grid {
    let input = input.trim();
    let (width, _) = input.bytes().find_position(|&c| c == b'\n').unwrap();
    let grid: Vec<u8> = input.lines()
        .flat_map(str::bytes)
        .collect();

    Grid::new(grid, width)
}

#[aoc(day12 part1)]
pub fn part1(grid: &Grid) -> usize {
    let result = grid.bfs();
    result[grid.end]
}

#[aoc(day12, part2)]
pub fn part2(grid: &Grid) -> usize {
    let result = grid.bfs_reverse();
    let (_, dist) = result.into_iter()
        .enumerate()
        .filter(|&(i, _)| grid.grid[i] == b'a')
        .min_by_key(|&(_, dist)| dist).unwrap();
    dist
}
