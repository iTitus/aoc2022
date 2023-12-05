use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn winning_score(&self, other: &Hand) -> u32 {
        match (self, other) {
            (a, b) if a == b => 3,
            (Hand::Rock, Hand::Paper) => 0,
            (Hand::Rock, Hand::Scissors) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (Hand::Paper, Hand::Scissors) => 0,
            (Hand::Scissors, Hand::Rock) => 0,
            (Hand::Scissors, Hand::Paper) => 6,
            _ => unreachable!(),
        }
    }
}

pub enum Left {
    A,
    B,
    C,
}

impl Left {
    pub fn to_hand(&self) -> Hand {
        match self {
            Left::A => Hand::Rock,
            Left::B => Hand::Paper,
            Left::C => Hand::Scissors,
        }
    }
}

impl FromStr for Left {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Left::A),
            "B" => Ok(Left::B),
            "C" => Ok(Left::C),
            _ => Err(()),
        }
    }
}

pub enum Right {
    X,
    Y,
    Z,
}

impl Right {
    pub fn to_hand(&self) -> Hand {
        match self {
            Right::X => Hand::Rock,
            Right::Y => Hand::Paper,
            Right::Z => Hand::Scissors,
        }
    }

    pub fn to_desired_hand(&self, other_hand: &Hand) -> Hand {
        match (self, other_hand) {
            (Self::X, Hand::Rock) => Hand::Scissors,
            (Self::X, Hand::Paper) => Hand::Rock,
            (Self::X, Hand::Scissors) => Hand::Paper,
            (Self::Y, other_hand) => *other_hand,
            (Self::Z, Hand::Rock) => Hand::Paper,
            (Self::Z, Hand::Paper) => Hand::Scissors,
            (Self::Z, Hand::Scissors) => Hand::Rock,
        }
    }
}

impl FromStr for Right {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Right::X),
            "Y" => Ok(Right::Y),
            "Z" => Ok(Right::Z),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Left, Right)> {
    input
        .lines()
        .map(|l| {
            let v = l.trim().splitn(2, ' ').collect_vec();
            (
                Left::from_str(v[0]).unwrap(),
                Right::from_str(v[1]).unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Left, Right)]) -> u32 {
    input.iter().map(|(l, r)| get_score_1(l, r)).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Left, Right)]) -> u32 {
    input.iter().map(|(l, r)| get_score_2(l, r)).sum()
}

fn get_score_1(l: &Left, r: &Right) -> u32 {
    let other_hand = l.to_hand();
    let my_hand = r.to_hand();
    my_hand.winning_score(&other_hand) + my_hand.score()
}

fn get_score_2(l: &Left, r: &Right) -> u32 {
    let other_hand = l.to_hand();
    let my_hand = r.to_desired_hand(&other_hand);
    my_hand.winning_score(&other_hand) + my_hand.score()
}
