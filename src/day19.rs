use std::collections::VecDeque;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Blueprint {
    index: u8,
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
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
    time_left: u8,
    ore_robot_count: u8,
    clay_robot_count: u8,
    obsidian_robot_count: u8,
    geode_robot_count: u8,
    ore_count: u8,
    clay_count: u8,
    obsidian_count: u8,
    geode_count: u8,
}

impl State {
    fn initial(time_left: u8) -> State {
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

    fn add_successors(&self, blueprint: &Blueprint, q: &mut VecDeque<State>) {
        if self.time_left > 0 {
            let ore_count = self.ore_count;
            let clay_count = self.clay_count;
            let obsidian_count = self.obsidian_count;

            for geode_robot_production in 0..=self.ore_count {
                if ore_count < geode_robot_production * blueprint.geode_robot_ore_cost || obsidian_count < geode_robot_production * blueprint.geode_robot_obsidian_cost {
                    break;
                }

                let ore_count = ore_count - geode_robot_production * blueprint.geode_robot_ore_cost;
                let obsidian_count = obsidian_count - geode_robot_production * blueprint.geode_robot_obsidian_cost;
                for obsidian_robot_production in 0..=self.ore_count {
                    if ore_count < obsidian_robot_production * blueprint.obsidian_robot_ore_cost || clay_count < obsidian_robot_production * blueprint.obsidian_robot_clay_cost {
                        break;
                    }

                    let ore_count = ore_count - obsidian_robot_production * blueprint.obsidian_robot_ore_cost;
                    let clay_count = clay_count - obsidian_robot_production * blueprint.obsidian_robot_clay_cost;
                    for clay_robot_production in 0..=self.ore_count {
                        if ore_count < clay_robot_production * blueprint.clay_robot_ore_cost {
                            break;
                        }

                        let ore_count = ore_count - clay_robot_production * blueprint.clay_robot_ore_cost;
                        for ore_robot_production in 0..=self.ore_count {
                            if ore_count < ore_robot_production * blueprint.ore_robot_ore_cost {
                                break;
                            }

                            let ore_count = ore_count - ore_robot_production * blueprint.ore_robot_ore_cost;
                            let mut s = *self;
                            s.time_left -= 1;
                            s.ore_robot_count += ore_robot_production;
                            s.clay_robot_count += clay_robot_production;
                            s.obsidian_robot_count += obsidian_robot_production;
                            s.geode_robot_count += geode_robot_production;
                            s.ore_count = ore_count + self.ore_robot_count;
                            s.clay_count = clay_count + self.clay_robot_count;
                            s.obsidian_count = obsidian_count + self.obsidian_robot_count;
                            s.geode_count += self.geode_robot_count;
                            println!("  succ: {s:?}");
                            q.push_back(s);
                        }
                    }
                }
            }
        }
    }

    fn max_geode_count_heuristic(&self) -> u32 {
        if self.time_left == 0 {
            self.geode_count as u32
        } else {
            (self.geode_count as u32) + (self.geode_robot_count as u32 * self.time_left as u32) + {
                let time_left_m1 = (self.time_left as u32) - 1;
                (time_left_m1 * time_left_m1 + time_left_m1) / 2
            }
        }
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> u32 {
    input.iter()
        .map(|blueprint| {
            let mut max_geode_count = 0;
            let mut visited = FxHashSet::default();
            let mut q = VecDeque::new();
            q.push_back(State::initial(24));
            while let Some(s) = q.pop_front() {
                if !visited.insert(s) {
                    continue;
                }

                println!("{s:?}");
                if (s.geode_count as u32) > max_geode_count {
                    max_geode_count = s.geode_count as u32;
                }

                if s.max_geode_count_heuristic() < max_geode_count {
                    continue;
                }

                s.add_successors(blueprint, &mut q);
            }

            (blueprint.index as u32) * max_geode_count
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(_: &[Blueprint]) -> usize {
    0
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
");
        assert_eq!(33, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
");
        assert_eq!(0, part2(&input))
    }
}
