use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct GridForest {
    trees: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl GridForest {
    pub fn new(trees: Vec<Vec<u8>>) -> GridForest {
        let height = trees.len();
        let width = trees[0].len();
        if trees.iter().any(|l| l.len() != width) {
            panic!("width mismatch");
        }

        GridForest {
            trees,
            width,
            height,
        }
    }

    fn get_raw(&self, x: usize, y: usize) -> u8 {
        self.trees[y][x]
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            panic!("index out of bounds");
        }

        self.get_raw(x, y)
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.get(x, y);

        if (0..x).map(|x| self.get_raw(x, y)).all(|t| t < tree) {
            return true;
        }

        if (x + 1..self.width)
            .map(|x| self.get_raw(x, y))
            .all(|t| t < tree)
        {
            return true;
        }

        if (0..y).map(|y| self.get_raw(x, y)).all(|t| t < tree) {
            return true;
        }

        if (y + 1..self.height)
            .map(|y| self.get_raw(x, y))
            .all(|t| t < tree)
        {
            return true;
        }

        false
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree = self.get(x, y);
        let mut score = 1;

        score *= x.min(
            (0..x)
                .rev()
                .map(|x| self.get_raw(x, y))
                .take_while(|t| *t < tree)
                .count()
                + 1,
        );
        score *= (self.width - x - 1).min(
            (x + 1..self.width)
                .map(|x| self.get_raw(x, y))
                .take_while(|t| *t < tree)
                .count()
                + 1,
        );
        score *= y.min(
            (0..y)
                .rev()
                .map(|y| self.get_raw(x, y))
                .take_while(|t| *t < tree)
                .count()
                + 1,
        );
        score *= (self.height - y - 1).min(
            (y + 1..self.height)
                .map(|y| self.get_raw(x, y))
                .take_while(|t| *t < tree)
                .count()
                + 1,
        );

        score
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> GridForest {
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let len = trees[0].len();
    if trees.iter().any(|l| l.len() != len) {
        panic!("length mismatch");
    }

    GridForest::new(trees)
}

#[aoc(day8, part1)]
pub fn part1(forest: &GridForest) -> usize {
    (0..forest.width)
        .flat_map(|x| (0..forest.height).map(move |y| (x, y)))
        .filter(|(x, y)| forest.is_visible(*x, *y))
        .count()
}

#[aoc(day8, part2)]
pub fn part2(forest: &GridForest) -> usize {
    (0..forest.width)
        .flat_map(|x| (0..forest.height).map(move |y| (x, y)))
        .map(|(x, y)| forest.scenic_score(x, y))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part2_1() {
        let forest = input_generator(
            r"
30373
25512
65332
33549
35390
",
        );
        assert_eq!(forest.scenic_score(2, 1), 4);
    }

    #[test]
    fn test_part2_2() {
        let forest = input_generator(
            r"
30373
25512
65332
33549
35390
",
        );
        assert_eq!(forest.scenic_score(2, 3), 8);
    }
}
