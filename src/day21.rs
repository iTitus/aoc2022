use std::str;

use aoc_runner_derive::{aoc, aoc_generator};
use num::{One, Rational64, Zero};
use pathfinding::prelude::*;
use rustc_hash::FxHashMap;

pub type Name = [u8; 4];
pub type Num = Rational64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Monkey {
    Num(Num),
    Add(Name, Name),
    Sub(Name, Name),
    Mul(Name, Name),
    Div(Name, Name),
}

impl Monkey {
    fn evaluate(&self, evaluated: &FxHashMap<Name, Num>) -> Num {
        match self {
            Monkey::Num(n) => *n,
            Monkey::Add(op1, op2) => evaluated[op1] + evaluated[op2],
            Monkey::Sub(op1, op2) => evaluated[op1] - evaluated[op2],
            Monkey::Mul(op1, op2) => evaluated[op1] * evaluated[op2],
            Monkey::Div(op1, op2) => evaluated[op1] / evaluated[op2],
        }
    }
}

const ROOT: Name = [b'r', b'o', b'o', b't'];
const HUMN: Name = [b'h', b'u', b'm', b'n'];

fn topo_sort(input: &FxHashMap<Name, Monkey>) -> Vec<Name> {
    let mut ordering = topological_sort(&[ROOT], |name: &Name| match input[name] {
        Monkey::Num(_) => { [None, None] }
        Monkey::Add(op1, op2) | Monkey::Sub(op1, op2) | Monkey::Mul(op1, op2) | Monkey::Div(op1, op2) => { [Some(op1), Some(op2)] }
    }.into_iter().flatten()).unwrap();
    ordering.reverse();
    ordering
}

fn evaluate_p1(input: &FxHashMap<Name, Monkey>) -> Num {
    let mut evaluated: FxHashMap<Name, Num> = FxHashMap::default();
    for name in topo_sort(input) {
        let eval = input[&name].evaluate(&evaluated);
        evaluated.insert(name, eval);
    }

    evaluated[&ROOT]
}

fn evaluate_p2(input: &FxHashMap<Name, Monkey>) -> Num {
    let root_monkey = &input[&ROOT];
    let fixed_root_monkey = match root_monkey {
        Monkey::Sub(_, _) => *root_monkey,
        Monkey::Add(op1, op2) | Monkey::Mul(op1, op2) | Monkey::Div(op1, op2) => Monkey::Sub(*op1, *op2),
        _ => panic!()
    };

    let ordering = topo_sort(input);
    let do_evaluation = |humn_override| {
        let humn_override = Monkey::Num(humn_override);
        let mut evaluated: FxHashMap<Name, Num> = FxHashMap::default();
        for name in &ordering {
            let mut monkey = &input[name];
            if name == &ROOT {
                monkey = &fixed_root_monkey;
            } else if name == &HUMN {
                monkey = &humn_override;
            }

            let eval = monkey.evaluate(&evaluated);
            evaluated.insert(*name, eval);
        }

        evaluated[&ROOT]
    };

    // using secant method to approximate a root
    let mut humn_override_0 = Num::zero();
    let mut result_0 = do_evaluation(humn_override_0);
    if result_0.is_zero() {
        return humn_override_0;
    }

    let mut humn_override_1 = Num::one();
    loop {
        let result_1 = do_evaluation(humn_override_1);
        if result_1.is_zero() {
            return humn_override_1;
        }

        (humn_override_0, result_0, humn_override_1) = (humn_override_1, result_1, humn_override_1 - result_1 * (humn_override_1 - humn_override_0) / (result_1 - result_0));
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> FxHashMap<Name, Monkey> {
    input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let bytes = l.as_bytes();
            let name: Name = bytes[..4].try_into().unwrap();

            let suffix = &bytes[6..];
            let monkey = if (b'0'..=b'9').contains(&suffix[0]) {
                Monkey::Num(str::from_utf8(suffix).unwrap().parse().unwrap())
            } else {
                let op1 = suffix[..4].try_into().unwrap();
                let op2 = suffix[7..].try_into().unwrap();
                match suffix[5] {
                    b'+' => Monkey::Add(op1, op2),
                    b'-' => Monkey::Sub(op1, op2),
                    b'*' => Monkey::Mul(op1, op2),
                    b'/' => Monkey::Div(op1, op2),
                    _ => panic!()
                }
            };
            (name, monkey)
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &FxHashMap<Name, Monkey>) -> Num {
    evaluate_p1(input)
}

#[aoc(day21, part2)]
pub fn part2(input: &FxHashMap<Name, Monkey>) -> Num {
    evaluate_p2(input)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
");
        assert_eq!(Num::from(152), part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
");
        assert_eq!(Num::from(301), part2(&input))
    }
}
