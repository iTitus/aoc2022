use std::num::NonZeroU32;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let all_calories = input
        .lines()
        .map(|l| {
            if l.is_empty() {
                0
            } else {
                NonZeroU32::from_str(l).unwrap().get()
            }
        }).collect_vec();
    let mut calories_grouped = vec![];
    let mut current_group: Vec<u32> = vec![];
    for calorie in all_calories {
        if calorie != 0 {
            current_group.push(calorie);
        } else {
            calories_grouped.push(current_group);
            current_group = vec![];
        }
    }

    if !current_group.is_empty() {
        calories_grouped.push(current_group);
    }

    calories_grouped
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|calories| calories.iter().sum())
        .sorted_by(|c1: &u32, c2: &u32| c1.cmp(c2).reverse())
        .take(3)
        .sum()
}
