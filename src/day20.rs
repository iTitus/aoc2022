use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn mix(input: &[isize], decryption_key: isize, iterations: usize) -> isize {
    let mut ans = (0..input.len()).collect_vec();
    for _ in 0..iterations {
        for (i, &n) in input.iter().enumerate() {
            let n = n * decryption_key;
            let pos = ans.iter().position(|&permuted_i| permuted_i == i).unwrap();
            ans.remove(pos);
            let new_i = (pos as isize + n).rem_euclid(ans.len() as isize) as usize;
            ans.insert(new_i, i);
        }
    }

    let zero_pos = input.iter().position(|&n| n == 0).unwrap();
    let zero_pos = ans.iter().position(|&n| n == zero_pos).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|n| n + zero_pos)
        .map(|n| (n as isize).rem_euclid(input.len() as isize) as usize)
        .map(|n| ans[n])
        .map(|n| input[n] * decryption_key)
        .sum()
}

#[aoc(day20, part1)]
pub fn part1(input: &[isize]) -> isize {
    mix(input, 1, 1)
}

#[aoc(day20, part2)]
pub fn part2(input: &[isize]) -> isize {
    mix(input, 811589153, 10)
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"1
2
-3
3
-2
0
4
");
        assert_eq!(3, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r"1
2
-3
3
-2
0
4
");
        assert_eq!(1623178306, part2(&input))
    }
}
