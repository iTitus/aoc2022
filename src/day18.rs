use std::collections::VecDeque;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet;

const NEIGHBORS: [Cube; 6] = [
    Cube { x: 1, y: 0, z: 0 },
    Cube { x: -1, y: 0, z: 0 },
    Cube { x: 0, y: 1, z: 0 },
    Cube { x: 0, y: -1, z: 0 },
    Cube { x: 0, y: 0, z: 1 },
    Cube { x: 0, y: 0, z: -1 },
];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    pub fn neighbors(&self) -> impl Iterator<Item = Cube> + '_ {
        NEIGHBORS.iter().map(|d| Cube {
            x: self.x + d.x,
            y: self.y + d.y,
            z: self.z + d.z,
        })
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y, z)) = s.splitn(3, ',').collect_tuple() {
            Ok(Cube {
                x: x.parse().map_err(|_| ())?,
                y: y.parse().map_err(|_| ())?,
                z: z.parse().map_err(|_| ())?,
            })
        } else {
            Err(())
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> FxHashSet<Cube> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day18 part1)]
pub fn part1(cubes: &FxHashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|c| c.neighbors().filter(|n| !cubes.contains(n)).count())
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(cubes: &FxHashSet<Cube>) -> usize {
    let min_x = cubes.iter().map(|c| c.x).min().unwrap() - 1;
    let max_x = cubes.iter().map(|c| c.x).max().unwrap() + 1;
    let min_y = cubes.iter().map(|c| c.y).min().unwrap() - 1;
    let max_y = cubes.iter().map(|c| c.y).max().unwrap() + 1;
    let min_z = cubes.iter().map(|c| c.z).min().unwrap() - 1;
    let max_z = cubes.iter().map(|c| c.z).max().unwrap() + 1;

    let mut outside_air = FxHashSet::default();
    let mut q = VecDeque::new();
    q.push_back(Cube {
        x: min_x,
        y: min_y,
        z: min_z,
    });
    while let Some(c) = q.pop_front() {
        if outside_air.insert(c) {
            q.extend(
                c.neighbors()
                    .filter(|n| {
                        n.x >= min_x
                            && n.x <= max_x
                            && n.y >= min_y
                            && n.y <= max_y
                            && n.z >= min_z
                            && n.z <= max_z
                    })
                    .filter(|n| !cubes.contains(n)),
            )
        }
    }

    cubes
        .iter()
        .map(|c| c.neighbors().filter(|n| outside_air.contains(n)).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(
            r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
        );
        assert_eq!(64, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(
            r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
        );
        assert_eq!(58, part2(&input))
    }
}
