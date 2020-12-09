use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

struct SlidingWindow {
    size: usize,
    numbers: VecDeque<u64>,
    cache: HashSet<u64>,
}

impl SlidingWindow {
    fn new(size: usize) -> Self {
        Self {
            size,
            numbers: VecDeque::with_capacity(25),
            cache: HashSet::with_capacity(25),
        }
    }

    fn add(&mut self, value: u64) {
        if self.is_full() {
            let removed = self.numbers.pop_front().unwrap();
            self.numbers.push_back(value);
            self.cache.remove(&removed);
            self.cache.insert(value);
        } else {
            self.numbers.push_back(value);
            self.cache.insert(value);
        }
    }

    fn is_full(&self) -> bool {
        self.numbers.len() == self.size
    }

    fn is_two_sum(&self, value: u64) -> bool {
        for &number in self.cache.iter() {
            if number < value && self.cache.contains(&(value - number)) {
                return true;
            }
        }

        false
    }
}

#[aoc_generator(day9)]
pub fn parser(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    let mut window = SlidingWindow::new(25);
    for &number in input.iter() {
        if window.is_full() && !window.is_two_sum(number) {
            return number;
        }

        window.add(number);
    }

    unreachable!();
}

struct TotalWindow {
    numbers: VecDeque<u64>,
    total: u64,
}

impl TotalWindow {
    fn new() -> Self {
        Self {
            numbers: VecDeque::new(),
            total: 0,
        }
    }

    fn add(&mut self, value: u64) {
        self.numbers.push_back(value);
        self.total += value;
    }

    fn bump(&mut self) {
        let removed = self.numbers.pop_front().unwrap();
        self.total -= removed;
    }

    fn min_max(self) -> (u64, u64) {
        self.numbers
            .into_iter()
            .fold((u64::MAX, u64::MIN), |(min, max), num| {
                (
                    if num < min { num } else { min },
                    if num > max { num } else { max },
                )
            })
    }
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let part1_solution = 22477624; // Solution from running Part 1
    let mut window = TotalWindow::new();

    for &number in input.iter() {
        window.add(number);

        loop {
            match window.total.cmp(&part1_solution) {
                Ordering::Equal => {
                    let (min, max) = window.min_max();
                    return min + max;
                }
                Ordering::Greater => {
                    window.bump();
                }
                Ordering::Less => {
                    break;
                }
            }
        }
    }

    unreachable!();
}
