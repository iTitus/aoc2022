use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Rucksack = (HashSet<u32>, HashSet<u32>);

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| (&l[0..(l.len() / 2)], &l[(l.len() / 2)..l.len()]))
        .map(|(l, r)| (letters_to_numbers(l), letters_to_numbers(r)))
        .collect()
}

fn letters_to_numbers(s: &str) -> HashSet<u32> {
    s.chars().map(letter_to_number).collect()
}

fn letter_to_number(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => (c as u32 - 'A' as u32) + 27,
        _ => panic!()
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rucksack]) -> u32 {
    input.iter()
        .map(|(l, r)| l.intersection(r).at_most_one().unwrap().unwrap())
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Rucksack]) -> u32 {
    input.iter()
        .chunks(3)
        .into_iter()
        .map(|g| {
            let mut rucksacks = g.into_iter()
                .map(|(l, r)| l.union(r).collect::<HashSet<_>>());
            let first = rucksacks.next().unwrap();
            rucksacks.fold(first, |a, e| &a & &e)
                .into_iter()
                .at_most_one().unwrap().unwrap()
        })
        .sum()
}
