use std::cmp::Ordering;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<Packet>> = vec![];
        let mut it = s.chars().peekable();
        while let Some(c) = it.peek() {
            match c {
                '[' => {
                    stack.push(vec![]);
                    it.next();
                }
                ']' => {
                    let tos = Packet::List(stack.pop().ok_or(())?);
                    it.next();
                    match stack.last_mut() {
                        Some(new_tos) => new_tos.push(tos),
                        None => return it.next().map_or_else(|| Ok(tos), |_| Err(())),
                    }
                }
                '0'..='9' => stack.last_mut().ok_or(())?.push(Packet::Int(
                    it.peeking_take_while(|c| ('0'..='9').contains(c))
                        .collect::<String>()
                        .parse()
                        .map_err(|_| ())?,
                )),
                ',' | ' ' => {
                    it.next();
                }
                _ => break,
            }
        }

        Err(())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Int(_), Packet::List(b)) => {
                if b.is_empty() {
                    Ordering::Greater
                } else {
                    self.cmp(&b[0]).then_with(|| 1.cmp(&b.len()))
                }
            }
            (Packet::List(a), Packet::Int(_)) => {
                if a.is_empty() {
                    Ordering::Less
                } else {
                    a[0].cmp(other).then_with(|| a.len().cmp(&1))
                }
            }
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Packet> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day13 part1)]
pub fn part1(packets: &[Packet]) -> usize {
    packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, chunk)| chunk[0] < chunk[1])
        .map(|(n, _)| n + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(packets: &[Packet]) -> usize {
    let mut packets = packets.to_vec();
    let div1: Packet = "[[2]]".parse().unwrap();
    let div2: Packet = "[[6]]".parse().unwrap();
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    (packets.iter().position(|p| p == &div1).unwrap() + 1)
        * (packets.iter().position(|p| p == &div2).unwrap() + 1)
}
