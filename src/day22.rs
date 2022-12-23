use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
pub fn input_generator(_input: &str) -> () {
    ()
}

#[aoc(day22, part1)]
pub fn part1(_input: &()) -> usize {
    0
}

#[aoc(day22, part2)]
pub fn part2(_input: &()) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
");
        assert_eq!(0, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
");
        assert_eq!(0, part2(&input))
    }
}
