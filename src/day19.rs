use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Blueprint {
    index: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
        }

        let c = RE.captures(s).unwrap();
        Ok(Blueprint {
            index: c[1].parse().map_err(|_| ())?,
            ore_robot_ore_cost: c[2].parse().map_err(|_| ())?,
            clay_robot_ore_cost: c[3].parse().map_err(|_| ())?,
            obsidian_robot_ore_cost: c[4].parse().map_err(|_| ())?,
            obsidian_robot_clay_cost: c[5].parse().map_err(|_| ())?,
            geode_robot_ore_cost: c[6].parse().map_err(|_| ())?,
            geode_robot_obsidian_cost: c[7].parse().map_err(|_| ())?,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    time_left: u32,
    ore_robot_count: u32,
    clay_robot_count: u32,
    obsidian_robot_count: u32,
    geode_robot_count: u32,
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,
}

impl State {
    fn initial(time_left: u32) -> State {
        State {
            time_left,
            ore_robot_count: 1,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
            geode_robot_count: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
        }
    }

    fn add_successors(&self, blueprint: &Blueprint, q: &mut Vec<State>) {
        if self.time_left == 0 {
            return;
        }

        fn calc_wait_time<T: Integer>(wanted: T, existing: T, production: T) -> T {
            if wanted <= existing {
                T::zero()
            } else {
                (wanted - existing).div_ceil(&production)
            }
        }

        if (self.obsidian_count >= blueprint.geode_robot_obsidian_cost
            || self.obsidian_robot_count > 0)
            && (self.ore_count >= blueprint.geode_robot_ore_cost || self.ore_robot_count > 0)
        {
            let wait_time = 1 + calc_wait_time(
                blueprint.geode_robot_obsidian_cost,
                self.obsidian_count,
                self.obsidian_robot_count,
            )
            .max(calc_wait_time(
                blueprint.geode_robot_ore_cost,
                self.ore_count,
                self.ore_robot_count,
            ));
            if wait_time <= self.time_left {
                q.push(State {
                    time_left: self.time_left - wait_time,
                    geode_robot_count: self.geode_robot_count + 1,
                    ore_count: self.ore_count + wait_time * self.ore_robot_count
                        - blueprint.geode_robot_ore_cost,
                    clay_count: self.clay_count + wait_time * self.clay_robot_count,
                    obsidian_count: self.obsidian_count + wait_time * self.obsidian_robot_count
                        - blueprint.geode_robot_obsidian_cost,
                    geode_count: self.geode_count + wait_time * self.geode_robot_count,
                    ..*self
                })
            }
        }

        if (self.clay_count >= blueprint.obsidian_robot_clay_cost || self.clay_robot_count > 0)
            && (self.ore_count >= blueprint.obsidian_robot_ore_cost || self.ore_robot_count > 0)
        {
            let wait_time = 1 + calc_wait_time(
                blueprint.obsidian_robot_clay_cost,
                self.clay_count,
                self.clay_robot_count,
            )
            .max(calc_wait_time(
                blueprint.obsidian_robot_ore_cost,
                self.ore_count,
                self.ore_robot_count,
            ));
            if wait_time <= self.time_left {
                q.push(State {
                    time_left: self.time_left - wait_time,
                    obsidian_robot_count: self.obsidian_robot_count + 1,
                    ore_count: self.ore_count + wait_time * self.ore_robot_count
                        - blueprint.obsidian_robot_ore_cost,
                    clay_count: self.clay_count + wait_time * self.clay_robot_count
                        - blueprint.obsidian_robot_clay_cost,
                    obsidian_count: self.obsidian_count + wait_time * self.obsidian_robot_count,
                    geode_count: self.geode_count + wait_time * self.geode_robot_count,
                    ..*self
                })
            }
        }

        if self.ore_count >= blueprint.clay_robot_ore_cost || self.ore_robot_count > 0 {
            let wait_time = 1 + calc_wait_time(
                blueprint.clay_robot_ore_cost,
                self.ore_count,
                self.ore_robot_count,
            );
            if wait_time <= self.time_left {
                q.push(State {
                    time_left: self.time_left - wait_time,
                    clay_robot_count: self.clay_robot_count + 1,
                    ore_count: self.ore_count + wait_time * self.ore_robot_count
                        - blueprint.clay_robot_ore_cost,
                    clay_count: self.clay_count + wait_time * self.clay_robot_count,
                    obsidian_count: self.obsidian_count + wait_time * self.obsidian_robot_count,
                    geode_count: self.geode_count + wait_time * self.geode_robot_count,
                    ..*self
                })
            }
        }

        if self.ore_count >= blueprint.ore_robot_ore_cost || self.ore_robot_count > 0 {
            let wait_time = 1 + calc_wait_time(
                blueprint.ore_robot_ore_cost,
                self.ore_count,
                self.ore_robot_count,
            );
            if wait_time <= self.time_left {
                q.push(State {
                    time_left: self.time_left - wait_time,
                    ore_robot_count: self.ore_robot_count + 1,
                    ore_count: self.ore_count + wait_time * self.ore_robot_count
                        - blueprint.ore_robot_ore_cost,
                    clay_count: self.clay_count + wait_time * self.clay_robot_count,
                    obsidian_count: self.obsidian_count + wait_time * self.obsidian_robot_count,
                    geode_count: self.geode_count + wait_time * self.geode_robot_count,
                    ..*self
                })
            }
        }
    }

    fn geode_count_upper_bound(&self) -> u32 {
        let production = self.geode_count + self.geode_robot_count * self.time_left;
        if self.time_left == 0 {
            production
        } else {
            production + {
                let time_left_m1 = self.time_left - 1;
                (time_left_m1 * time_left_m1 + time_left_m1) / 2
            }
        }
    }
}

fn max_geode_count(time_left: u32, blueprint: &Blueprint) -> u32 {
    let mut max_geode_count = 0;
    let mut q = vec![];
    q.push(State::initial(time_left));
    while let Some(s) = q.pop() {
        if s.geode_count > max_geode_count {
            max_geode_count = s.geode_count;
        }

        if s.geode_count_upper_bound() <= max_geode_count {
            continue;
        }

        s.add_successors(blueprint, &mut q);
    }

    max_geode_count
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> u32 {
    input
        .iter()
        .map(|blueprint| (blueprint.index as u32) * max_geode_count(24, blueprint))
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Blueprint]) -> u32 {
    input
        .iter()
        .take(3)
        .map(|blueprint| max_geode_count(32, blueprint))
        .product()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(
            r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
",
        );
        assert_eq!(33, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(
            r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
",
        );
        assert_eq!(3472, part2(&input))
    }
}
