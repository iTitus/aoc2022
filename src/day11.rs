use std::cmp::Reverse;
use std::collections::VecDeque;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Sub(u64),
    Mul(u64),
    Div(u64),
    Mod(u64),
    Square,
}

impl Operation {
    fn apply(&self, n: &u64) -> u64 {
        match self {
            Operation::Add(m) => n + m,
            Operation::Sub(m) => n - m,
            Operation::Mul(m) => n * m,
            Operation::Div(m) => n / m,
            Operation::Mod(m) => n % m,
            Operation::Square => n * n,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old * old" {
            Ok(Self::Square)
        } else if let Some(n) = s.strip_prefix("old + ") {
            Ok(Self::Add(n.parse().map_err(|_| ())?))
        } else if let Some(n) = s.strip_prefix("old - ") {
            Ok(Self::Sub(n.parse().map_err(|_| ())?))
        } else if let Some(n) = s.strip_prefix("old * ") {
            Ok(Self::Mul(n.parse().map_err(|_| ())?))
        } else if let Some(n) = s.strip_prefix("old / ") {
            Ok(Self::Div(n.parse().map_err(|_| ())?))
        } else if let Some(n) = s.strip_prefix("old % ") {
            Ok(Self::Mod(n.parse().map_err(|_| ())?))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisibility_test: u64,
    true_target: usize,
    false_target: usize,
    inspections: u64,
}

#[derive(Debug, Clone)]
pub struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    pub fn do_rounds<const ROUNDS: usize, const DIVISOR: u64>(&mut self) -> u64 {
        // could use lcm here, but all the divisibility_test numbers are prime
        let modulus: u64 = self.monkeys.iter().map(|m| m.divisibility_test).product();
        for _ in 0..ROUNDS {
            for i in 0..self.monkeys.len() {
                while let Some(item) = self.monkeys[i].items.pop_front() {
                    self.monkeys[i].inspections += 1;
                    let item = (self.monkeys[i].operation.apply(&item) / DIVISOR) % modulus;
                    let target = if item % self.monkeys[i].divisibility_test == 0 {
                        self.monkeys[i].true_target
                    } else {
                        self.monkeys[i].false_target
                    };
                    self.monkeys[target].items.push_back(item);
                }
            }
        }

        self.monkeys.iter()
            .map(|m| m.inspections)
            .sorted_by_key(|&n| Reverse(n))
            .take(2)
            .product()
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Monkeys {
    let re = Regex::new(
        r"^Monkey (\d+):
  Starting items: (\d+(?:, \d+)*)
  Operation: new = ([old +*0-9]+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)$"
    ).unwrap();
    Monkeys {
        monkeys: input.trim().split("\n\n")
            .map(|s| {
                let caps = re.captures(s).unwrap();
                // let index: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let items: VecDeque<u64> = caps.get(2).unwrap().as_str().split(", ").map(|n| n.parse().unwrap()).collect();
                let operation: Operation = caps.get(3).unwrap().as_str().parse().unwrap();
                let divisibility_test: u64 = caps.get(4).unwrap().as_str().parse().unwrap();
                let true_target: usize = caps.get(5).unwrap().as_str().parse().unwrap();
                let false_target: usize = caps.get(6).unwrap().as_str().parse().unwrap();
                Monkey {
                    items,
                    operation,
                    divisibility_test,
                    true_target,
                    false_target,
                    inspections: 0,
                }
            })
            .collect()
    }
}

#[aoc(day11, part1)]
pub fn part1(monkeys: &Monkeys) -> u64 {
    let mut monkeys = monkeys.clone();
    monkeys.do_rounds::<20, 3>()
}

#[aoc(day11, part2)]
pub fn part2(monkeys: &Monkeys) -> u64 {
    let mut monkeys = monkeys.clone();
    monkeys.do_rounds::<10000, 1>()
}
