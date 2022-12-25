use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

fn from_snafu(s: &str) -> i64 {
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

    let mut n = 0;
    let digits = s.as_bytes();
    for &c in digits {
        n = 5 * n + from_snafu_digit(c as char) as i64;
    }

    n
}

fn to_snafu(mut n: i64) -> String {
    fn to_snafu_digit(d: i64) -> char {
        match d {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!()
        }
    }

    if n < 0 {
        unimplemented!()
    } else if n == 0 {
        return "0".to_string();
    }

    let mut s = String::new();
    while n > 0 {
        let c = to_snafu_digit((n + 2) % 5);
        s.insert(0, c);
        n = (n + 2) / 5;
    }

    s
}

#[aoc(day25, part1)]
pub fn part1(input: &[String]) -> String {
    to_snafu(
        input.iter()
            .map(|s| from_snafu(s))
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_from_0_to_ten() {
        assert_eq!(0, from_snafu("0"));
        assert_eq!(1, from_snafu("1"));
        assert_eq!(2, from_snafu("2"));
        assert_eq!(3, from_snafu("1="));
        assert_eq!(4, from_snafu("1-"));
        assert_eq!(5, from_snafu("10"));
        assert_eq!(6, from_snafu("11"));
        assert_eq!(7, from_snafu("12"));
        assert_eq!(8, from_snafu("2="));
        assert_eq!(9, from_snafu("2-"));
        assert_eq!(10, from_snafu("20"));
    }

    #[test]
    fn test_to_0_to_ten() {
        assert_eq!("0", to_snafu(0));
        assert_eq!("1", to_snafu(1));
        assert_eq!("2", to_snafu(2));
        assert_eq!("1=", to_snafu(3));
        assert_eq!("1-", to_snafu(4));
        assert_eq!("10", to_snafu(5));
        assert_eq!("11", to_snafu(6));
        assert_eq!("12", to_snafu(7));
        assert_eq!("2=", to_snafu(8));
        assert_eq!("2-", to_snafu(9));
        assert_eq!("20", to_snafu(10));
    }
}
