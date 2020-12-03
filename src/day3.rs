use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::successors;

#[aoc_generator(day3)]
pub fn parser(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> u64 {
    find_trees((3, 1), input)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> u64 {
    let mut product = 1;
    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        product *= find_trees(*slope, input);
    }
    product
}

fn find_trees(slope: (usize, usize), input: &[Vec<bool>]) -> u64 {
    let bottom = input.len();
    let width = input[0].len();

    successors(Some((0, 0)), |(x, y)| {
        let new_y = y + slope.1;
        if new_y < bottom {
            let new_x = (x + slope.0) % width;
            Some((new_x, new_y))
        } else {
            None
        }
    })
    .map(|(x, y)| if input[y][x] { 1 } else { 0 })
    .sum()
}
