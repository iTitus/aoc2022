use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day24)]
pub fn input_generator(_input: &str) -> () {
    ()
}

#[aoc(day24, part1)]
pub fn part1(_input: &()) -> usize {
    0
}

#[aoc(day24, part2)]
pub fn part2(_input: &()) -> usize {
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

    #[test]
    fn test_2() {
        let input = input_generator(r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
");
        assert_eq!(0, part2(&input))
    }
}
