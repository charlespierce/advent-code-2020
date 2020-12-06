use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let set: HashSet<_> = group.chars().filter(|c| c.is_alphabetic()).collect();
            set.len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let (count, answers) =
                group
                    .lines()
                    .fold((0, HashMap::new()), |(count, mut answers), person| {
                        for question in person.chars() {
                            *answers.entry(question).or_insert(0) += 1;
                        }
                        (count + 1, answers)
                    });

            answers.values().filter(|c| **c == count).count()
        })
        .sum()
}
