use std::cmp::Reverse;
use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

const START: [u8; 2] = [b'A', b'A'];

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Valve> {
    type NameT = [u8; 2];
    fn parse_name(name: &str) -> NameT {
        assert_eq!(name.len(), 2);
        let b = name.as_bytes();
        [b[0], b[1]]
    }

    struct ParseValve {
        name: NameT,
        index: usize,
        flow: u32,
        connections: Vec<NameT>,
    }

    let r = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)$").unwrap();
    let mut valves: FxHashMap<NameT, ParseValve> = input.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let c = r.captures(l).unwrap();
            let name = parse_name(&c[1]);
            let flow = c[2].parse().unwrap();
            let connections = c[3].split(", ").map(parse_name).collect();
            (name,
             ParseValve {
                 name,
                 index: 0,
                 flow,
                 connections,
             }
            )
        })
        .collect();

    // order will be [ relevant node 1, ..., relevant node m, AA, ..]
    valves.values_mut()
        .sorted_by_key(|node| (Reverse(node.flow), node.name))
        .enumerate()
        .for_each(|(n, node)| node.index = n);

    fn bfs(nodes: &FxHashMap<NameT, ParseValve>, source: &ParseValve) -> FxHashMap<usize, u32> {
        let mut dist: FxHashMap<usize, u32> = nodes.values().map(|n| (n.index, u32::MAX)).collect();
        *dist.get_mut(&source.index).unwrap() = 0;

        let mut q = VecDeque::new();
        q.push_back(source);

        while let Some(n) = q.pop_front() {
            let from_dist = dist[&n.index];
            let new_dist = from_dist + 1;
            for to in &nodes[&n.name].connections {
                let to = &nodes[to];
                let dist_value = dist.get_mut(&to.index).unwrap();
                if new_dist < *dist_value {
                    *dist_value = new_dist;
                    q.push_back(to);
                }
            }
        }

        dist
    }

    let sorted_valves: Vec<&ParseValve> = valves.values()
        .sorted_by_key(|node| node.index)
        .collect();
    sorted_valves.iter()
        .filter(|node| node.flow > 0 || node.name == START)
        .map(|node| {
            let mut dist = bfs(&valves, node);
            dist.retain(|n, _| sorted_valves[*n].flow > 0);
            Valve {
                flow: node.flow,
                connections: dist,
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Valve {
    flow: u32,
    connections: FxHashMap<usize, u32>,
}

fn gain(time_left: u32, d: u32, flow: u32) -> u32 {
    if d + 1 >= time_left {
        0
    } else {
        flow * (time_left - d - 1)
    }
}

#[aoc(day16 part1)]
pub fn part1(valves: &[Valve]) -> u32 {
    assert!(!valves.is_empty() && valves.len() <= 63);
    let start = valves.len() - 1;

    #[derive(Debug, Clone, Copy)]
    struct State {
        time_left: u32,
        total_pressure: u32,
        pos: usize,
        open: u64,
    }

    let mut max = 0;
    let mut q = VecDeque::new();
    q.push_back(State {
        time_left: 30,
        total_pressure: 0,
        pos: start,
        open: 0,
    });

    while let Some(state) = q.pop_front() {
        if state.total_pressure > max {
            max = state.total_pressure; // new max found
        }

        // branch pruning
        if state.time_left > 1 {
            let max_pressure_to_get: u32 = valves[state.pos].connections.iter()
                .filter(|(n, _)| state.open & (1 << **n) == 0)
                .map(|(n, d)| gain(state.time_left, *d, valves[*n].flow))
                .sum();
            if state.total_pressure + max_pressure_to_get <= max {
                continue;
            }
        }

        // iterate over all unopened valves you can travel to and open in time (sorted by expected gain)
        for (n, d) in valves[state.pos].connections.iter()
            .filter(|(n, d)| **d + 1 < state.time_left && (state.open & (1 << **n)) == 0)
        {
            let mut new_state = state;
            new_state.time_left -= *d + 1;
            new_state.total_pressure += gain(state.time_left, *d, valves[*n].flow);
            new_state.pos = *n;
            new_state.open |= 1 << n;
            q.push_back(new_state);
        }
    }

    max
}

#[aoc(day16, part2)]
pub fn part2(valves: &[Valve]) -> u32 {
    assert!(!valves.is_empty() && valves.len() <= 63);
    let start = valves.len() - 1;

    #[derive(Debug, Clone, Copy)]
    struct State {
        time_left1: u32,
        time_left2: u32,
        total_pressure: u32,
        pos1: usize,
        pos2: usize,
        open: u64,
    }

    let mut max = 0;
    let mut q = VecDeque::new();
    q.push_back(State {
        time_left1: 26,
        time_left2: 26,
        total_pressure: 0,
        pos1: start,
        pos2: start,
        open: 0,
    });

    while let Some(state) = q.pop_front() {
        if state.total_pressure > max {
            max = state.total_pressure; // new max found
        }

        if state.time_left1 <= 1 && state.time_left2 <= 1 {
            continue;
        }

        // branch pruning, can use idx because valves is sorted by flow rate
        let mut idx = 0;
        let max_pressure_to_get: u32 = valves.iter()
            .enumerate()
            .filter(|(n, v)| v.flow > 0 && state.open & (1 << *n) == 0)
            .map(|(n, v)| {
                let d1 = valves[state.pos1].connections[&n];
                let d2 = valves[state.pos2].connections[&n];
                idx += 1;
                gain(if idx < state.time_left1 { state.time_left1 - (idx - 1) } else { 0 }, d1, v.flow).max(gain(if idx < state.time_left2 { state.time_left2 - (idx - 1) } else { 0 }, d2, v.flow))
            })
            .sum();
        if state.total_pressure + max_pressure_to_get <= max {
            continue;
        }

        // for pos1: iterate over all unopened valves you can travel to and open in time
        for (n, d) in valves[state.pos1].connections.iter()
            .filter(|(n, d)| **d + 1 < state.time_left1 && (state.open & (1 << **n)) == 0 && **d <= valves[state.pos2].connections[*n])
        {
            let mut new_state = state;
            new_state.time_left1 -= *d + 1;
            new_state.total_pressure += gain(state.time_left1, *d, valves[*n].flow);
            new_state.pos1 = *n;
            new_state.open |= 1 << n;
            q.push_back(new_state);
        }

        if state.pos1 != state.pos2 {
            // for pos2: iterate over all unopened valves you can travel to and open in time
            for (n, d) in valves[state.pos2].connections.iter()
                .filter(|(n, d)| **d + 1 < state.time_left2 && (state.open & (1 << **n)) == 0 && **d <= valves[state.pos1].connections[*n])
            {
                let mut new_state = state;
                new_state.time_left2 -= *d + 1;
                new_state.total_pressure += gain(state.time_left2, *d, valves[*n].flow);
                new_state.pos2 = *n;
                new_state.open |= 1 << n;
                q.push_back(new_state);
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_1() {
        let input = input_generator(r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
");
        assert_eq!(1651, part1(&input))
    }

    #[test]
    fn test_2() {
        let input = input_generator(r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
");
        assert_eq!(1707, part2(&input))
    }
}

