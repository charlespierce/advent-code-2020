use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13, part1)]
pub fn parser(input: &str) -> (u32, Vec<u32>) {
    let mut parts = input.lines();
    let timestamp = parts.next().unwrap().parse().unwrap();
    let busses = parts
        .next()
        .unwrap()
        .split(',')
        .filter_map(|b| b.parse().ok())
        .collect();

    (timestamp, busses)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(u32, Vec<u32>)) -> u32 {
    let mut min = u32::MAX;
    let mut id = 0;
    let timestamp = input.0;

    for &value in &input.1 {
        let wait = value - (timestamp % value);
        if wait < min {
            min = wait;
            id = value;
        }
    }

    min * id
}

#[aoc_generator(day13, part2)]
pub fn parser_part2(input: &str) -> Vec<Option<u64>> {
    input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|b| b.parse().ok())
        .collect()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Option<u64>]) -> u64 {
    let mut timestamp = input[0].unwrap();
    let mut multiple = timestamp;

    for (index, &bus) in input.iter().enumerate().skip(1) {
        let index = index as u64;
        if let Some(id) = bus {
            loop {
                if (timestamp + index) % id == 0 {
                    break;
                }
                timestamp += multiple;
            }
            multiple *= id;
        }
    }

    timestamp
}
