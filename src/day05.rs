use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type Move = (usize, usize, usize);

#[derive(Debug, Clone)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn do_multi_move(&mut self, (count, from, to): &Move) {
        let from_stack = &mut self.stacks[*from];
        let len = from_stack.len();
        let cs = from_stack.drain((len - count)..).collect_vec();
        self.stacks[*to].extend_from_slice(&cs);
    }

    pub fn do_move(&mut self, (count, from, to): &Move) {
        for _ in 0..*count {
            let c = self.stacks[*from].pop().unwrap();
            self.stacks[*to].push(c);
        }
    }

    pub fn top_str(&self) -> String {
        self.stacks.iter()
            .filter_map(|s| s.last())
            .collect()
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Stacks, Vec<Move>) {
    let (a, b): (&str, &str) = input.split_once("\n\n").unwrap();

    let stacks: Vec<Vec<char>> =
        {
            let lines = a.lines().collect_vec();
            // let's hope all chars and numbers will always be ascii and only one char long
            let cols = (lines.last().unwrap().len() + 1) / 4;

            fn to_col_idx(i: usize) -> usize {
                i * 4 + 1
            }

            (0..cols).into_iter()
                .map(to_col_idx)
                .map(|i| lines.iter()
                    .rev()
                    .skip(1)
                    .map(|s| s.as_bytes()[i] as char)
                    .filter(|c| *c != ' ')
                    .collect())
                .collect()
        };

    let moves: Vec<(usize, usize, usize)> = {
        let r = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        b.trim().lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                let c = r.captures(l).unwrap();
                let count: usize = c[1].parse().unwrap();
                let from: usize = c[2].parse().unwrap();
                let to: usize = c[3].parse().unwrap();
                if count < 1 || from < 1 || from > stacks.len() || to < 1 || to > stacks.len() {
                    panic!();
                }
                (count, from - 1, to - 1)
            })
            .collect()
    };

    (Stacks { stacks }, moves)
}

#[aoc(day5, part1)]
pub fn part1((stacks, moves): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();
    for op in moves {
        stacks.do_move(op);
    }

    stacks.top_str()
}

#[aoc(day5, part2)]
pub fn part2((stacks, moves): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();
    for op in moves {
        stacks.do_multi_move(op);
    }

    stacks.top_str()
}
