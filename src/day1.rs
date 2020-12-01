use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn parser(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut seen = HashSet::new();

    for num in input {
        let complement = 2020 - *num;
        if seen.contains(&complement) {
            return complement * *num;
        } else {
            seen.insert(*num);
        }
    }

    panic!("No solution found!");
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut seen = HashSet::new();

    for num in input {
        for other in seen.iter() {
            let total = *num + *other;
            if total < 2020 {
                let complement = 2020 - total;
                if seen.contains(&complement) {
                    println!("{} + {} + {} = 2020", complement, *num, *other);
                    return complement * *num * *other;
                }
            }
        }
        seen.insert(*num);
    }
    panic!("No solution found!");
}
