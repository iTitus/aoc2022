use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Interval = (u32, u32);
type Pair = (Interval, Interval);

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            l.splitn(2, ',')
                .map(|range| {
                    let (lower, upper) = range
                        .splitn(2, '-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    if lower > upper {
                        panic!()
                    }
                    (lower, upper)
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn fully_contains(a: &Interval, b: &Interval) -> bool {
    b.0 >= a.0 && b.1 <= a.1
}

fn overlaps(a: &Interval, b: &Interval) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}

#[aoc(day4, part1)]
pub fn part1(input: &[Pair]) -> usize {
    input
        .iter()
        .filter(|(l, r)| fully_contains(l, r) || fully_contains(r, l))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Pair]) -> usize {
    input.iter().filter(|(l, r)| overlaps(l, r)).count()
}
