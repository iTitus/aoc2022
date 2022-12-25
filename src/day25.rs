use core::panicking::panic;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

fn from_snafu_digit(c: char) -> i8 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!()
    }
}

fn from_snafu(s: &str) -> i64 {
    let mut n = 0;
    let mut base = 1;
    for c in s.chars().rev() {
        n += base * from_snafu_digit(c) as i64;
        base *= 5;
    }

    n
}

fn to_snafu(n: i64) -> String {
    if n < 0 {
        unimplemented!()
    } else if n == 0 {
        return "0".to_string();
    }

    let mut s = String::new();
    let mut base = 1;
    while n > 0 {

    }

    s
}

fn to_snafu_digit(d: i8) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!()
    }
}

#[aoc(day25, part1)]
pub fn part1(_input: &[String]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
");
        assert_eq!(18, part1(&input))
    }
}
