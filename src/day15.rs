use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn tuning_frequency(&self) -> i64 {
        (self.x as i64) * 4000000 + (self.y as i64)
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<(Pos, Pos)> {
    let r = Regex::new(r"^Sensor at x=([+\-]?\d+), y=([+\-]?\d+): closest beacon is at x=([+\-]?\d+), y=([+\-]?\d+)$").unwrap();
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let c = r.captures(l).unwrap();
            (
                Pos {
                    x: c[1].parse().unwrap(),
                    y: c[2].parse().unwrap(),
                },
                Pos {
                    x: c[3].parse().unwrap(),
                    y: c[4].parse().unwrap(),
                },
            )
        })
        .collect()
}

fn no_beacon_pos<const Y: i32>(input: &[(Pos, Pos)]) -> usize {
    input
        .iter()
        .flat_map(|(sensor, beacon)| {
            let d = sensor.distance(beacon);
            let dy = Y.abs_diff(sensor.y);
            if d < dy {
                vec![]
            } else {
                let dx = (d - dy) as i32;
                ((sensor.x - dx)..=(sensor.x + dx))
                    .filter(|&x| Y != beacon.y || x != beacon.x)
                    .collect()
            }
        })
        .unique()
        .count()
}

fn corner_points(sensor: &Pos, d: u32) -> (Pos, Pos, Pos, Pos) {
    let d = d as i32;
    (
        Pos {
            x: sensor.x + d,
            y: sensor.y,
        },
        Pos {
            x: sensor.x - d,
            y: sensor.y,
        },
        Pos {
            x: sensor.x,
            y: sensor.y + d,
        },
        Pos {
            x: sensor.x,
            y: sensor.y - d,
        },
    )
}

fn diag_iter(a: &Pos, b: &Pos) -> impl Iterator<Item = Pos> {
    struct Iter {
        pos: Pos,
        end: Pos,
        done: bool,
        dx: i32,
        dy: i32,
    }

    impl Iterator for Iter {
        type Item = Pos;

        fn next(&mut self) -> Option<Self::Item> {
            if self.done {
                None
            } else {
                let pos = self.pos;
                if pos == self.end {
                    self.done = true;
                } else {
                    self.pos.x += self.dx;
                    self.pos.y += self.dy;
                }

                Some(pos)
            }
        }
    }

    Iter {
        pos: *a,
        end: *b,
        done: false,
        dx: (b.x - a.x).signum(),
        dy: (b.y - a.y).signum(),
    }
}

fn find_beacon_pos<const MAX: i32>(input: &[(Pos, Pos)]) -> Option<i64> {
    assert!(MAX >= 0);
    for (sensor, beacon) in input {
        let d = sensor.distance(beacon) + 1;
        let (e, w, s, n) = corner_points(sensor, d);

        for pos in diag_iter(&n, &w) {
            if pos.x >= 0
                && pos.x <= MAX
                && pos.y >= 0
                && pos.y <= MAX
                && input
                    .iter()
                    .all(|(sensor2, beacon2)| sensor2.distance(&pos) > sensor2.distance(beacon2))
            {
                return Some(pos.tuning_frequency());
            }
        }

        for pos in diag_iter(&n, &e) {
            if pos.x >= 0
                && pos.x <= MAX
                && pos.y >= 0
                && pos.y <= MAX
                && input
                    .iter()
                    .all(|(sensor2, beacon2)| sensor2.distance(&pos) > sensor2.distance(beacon2))
            {
                return Some(pos.tuning_frequency());
            }
        }

        for pos in diag_iter(&s, &w) {
            if pos.x >= 0
                && pos.x <= MAX
                && pos.y >= 0
                && pos.y <= MAX
                && input
                    .iter()
                    .all(|(sensor2, beacon2)| sensor2.distance(&pos) > sensor2.distance(beacon2))
            {
                return Some(pos.tuning_frequency());
            }
        }

        for pos in diag_iter(&s, &e) {
            if pos.x >= 0
                && pos.x <= MAX
                && pos.y >= 0
                && pos.y <= MAX
                && input
                    .iter()
                    .all(|(sensor2, beacon2)| sensor2.distance(&pos) > sensor2.distance(beacon2))
            {
                return Some(pos.tuning_frequency());
            }
        }
    }

    None
}

#[aoc(day15 part1)]
pub fn part1(input: &[(Pos, Pos)]) -> usize {
    no_beacon_pos::<2000000>(input)
}

#[aoc(day15, part2)]
pub fn part2(input: &[(Pos, Pos)]) -> i64 {
    find_beacon_pos::<4000000>(input).unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(
            r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
        );
        assert_eq!(26, no_beacon_pos::<10>(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(
            r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
        );
        assert_eq!(Some(56000011), find_beacon_pos::<20>(&input))
    }
}
