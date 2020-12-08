use aoc_runner_derive::aoc;
use std::collections::HashSet;

enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Operation {
    fn toggle(&mut self) {
        match self {
            Operation::Jmp(value) => {
                *self = Operation::Nop(*value);
            }
            Operation::Nop(value) => {
                *self = Operation::Jmp(*value);
            }
            _ => unreachable!(),
        }
    }
}

enum ExitMode {
    InfiniteLoop,
    Complete,
}

impl<'a> From<&'a str> for Operation {
    fn from(op: &'a str) -> Self {
        let value = op[4..].parse().unwrap();
        match &op[0..3] {
            "acc" => Operation::Acc(value),
            "jmp" => Operation::Jmp(value),
            "nop" => Operation::Nop(value),
            _ => unreachable!(),
        }
    }
}

pub struct Computer {
    ops: Vec<Operation>,
    cursor: usize,
    accumulator: i32,
}

impl Computer {
    fn new(ops: Vec<Operation>) -> Self {
        Self {
            ops,
            cursor: 0,
            accumulator: 0,
        }
    }

    fn run(&mut self) -> ExitMode {
        let mut seen = HashSet::new();

        loop {
            if self.cursor >= self.ops.len() {
                break ExitMode::Complete;
            }
            if seen.contains(&self.cursor) {
                break ExitMode::InfiniteLoop;
            } else {
                seen.insert(self.cursor);
            }

            match self.ops[self.cursor] {
                Operation::Acc(value) => {
                    self.accumulator += value;
                    self.cursor += 1;
                }
                Operation::Jmp(jump) => {
                    self.cursor = ((self.cursor as i32) + jump) as usize;
                }
                Operation::Nop(_) => {
                    self.cursor += 1;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.cursor = 0;
        self.accumulator = 0;
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut computer = Computer::new(input.lines().map(Operation::from).collect());
    match computer.run() {
        ExitMode::InfiniteLoop => computer.accumulator,
        _ => unreachable!(),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let ops: Vec<_> = input.lines().map(Operation::from).collect();
    let ops_to_toggle: Vec<_> = ops
        .iter()
        .enumerate()
        .filter_map(|(index, op)| match op {
            Operation::Jmp(_) | Operation::Nop(_) => Some(index),
            _ => None,
        })
        .collect();
    let mut computer = Computer::new(ops);

    for toggled in ops_to_toggle {
        computer.ops[toggled].toggle();

        match computer.run() {
            ExitMode::Complete => return computer.accumulator,
            ExitMode::InfiniteLoop => {
                computer.ops[toggled].toggle();
                computer.reset();
            }
        }
    }

    unreachable!();
}
