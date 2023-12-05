use aoc_runner_derive::aoc;
use itertools::Itertools;

fn find_first_distinct(input: &str, n: usize) -> usize {
    // once again we assume that the input is only ascii
    input
        .trim()
        .as_bytes()
        .windows(n)
        .position(|window| window.iter().all_unique())
        .map(|i| i + n)
        .unwrap()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    find_first_distinct(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    find_first_distinct(input, 14)
}
