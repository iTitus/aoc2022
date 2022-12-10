use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        if let Some(s) = s.strip_prefix("addx ") {
            return Ok(Instruction::Addx(s.parse().map_err(|_| ())?));
        }

        Err(())
    }
}

#[derive(Debug, Clone)]
pub struct Cpu {
    pub register_x: i32,
    pub cycle: u32,
    pc: u32,
    cycles_with_current_instruction: u32,
    instructions: Vec<Instruction>,
}

impl Cpu {
    pub fn new(instructions: Vec<Instruction>) -> Cpu {
        Cpu { register_x: 1, cycle: 0, pc: 0, cycles_with_current_instruction: 0, instructions }
    }

    pub fn has_instruction(&self) -> bool {
        (self.pc as usize) < self.instructions.len()
    }

    pub fn pre_cycle(&mut self) {
        self.cycle += 1;
        self.cycles_with_current_instruction += 1;
    }

    pub fn post_cycle(&mut self) {
        if self.cycles_with_current_instruction >= self.instructions[self.pc as usize].cycles() {
            self.execute_current_instruction();
            self.pc += 1;
            self.cycles_with_current_instruction = 0;
        }
    }

    fn execute_current_instruction(&mut self) {
        match self.instructions[self.pc as usize] {
            Instruction::Noop => {}
            Instruction::Addx(n) => self.register_x += n,
        }
    }

    pub fn signal_strength(&self) -> i32 {
        self.cycle as i32 * self.register_x
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Cpu {
    Cpu::new(input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect())
}

#[aoc(day10, part1)]
pub fn part1(cpu: &Cpu) -> i32 {
    let mut cpu = cpu.clone();
    let mut sum: i32 = 0;
    while cpu.has_instruction() {
        cpu.pre_cycle();
        if (20..=220).contains(&cpu.cycle) && (cpu.cycle - 20) % 40 == 0 {
            sum += cpu.signal_strength();
        }

        cpu.post_cycle();
    }

    sum
}

#[aoc(day10, part2)]
pub fn part2(cpu: &Cpu) -> String {
    let mut cpu = cpu.clone();
    let mut display = '\n'.to_string();
    while cpu.has_instruction() {
        cpu.pre_cycle();
        let pixel = (cpu.cycle as i32 - 1) % 40;
        display.push(if cpu.register_x.abs_diff(pixel) <= 1 { '\u{2588}' } else { ' ' });
        if pixel == 39 {
            display.push('\n');
        }
        cpu.post_cycle();
    }

    display
}
