use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn seat_id(&self) -> u32 {
        (self.row as u32) * 8 + (self.column as u32)
    }
}

fn binary_fold(acc: (u8, u8), value: &u8) -> (u8, u8) {
    let midpoint = acc.0 + ((acc.1 - acc.0) / 2);
    match value {
        b'F' | b'L' => (acc.0, midpoint),
        b'B' | b'R' => (midpoint + 1, acc.1),
        _ => unreachable!(),
    }
}

impl<'a> From<&'a [u8]> for Seat {
    fn from(specifier: &'a [u8]) -> Seat {
        let (row, _) = specifier.iter().take(7).fold((0, 127), binary_fold);
        let (column, _) = specifier.iter().skip(7).take(3).fold((0, 7), binary_fold);

        Seat { row, column }
    }
}

#[aoc_generator(day5)]
pub fn parser(input: &str) -> Vec<Seat> {
    input.lines().map(|l| l.as_bytes().into()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Seat]) -> u32 {
    input.iter().map(Seat::seat_id).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Seat]) -> u32 {
    let filled_seats: HashSet<_> = input.iter().map(Seat::seat_id).collect();

    // 935 was the maximum seat id from part 1
    for seat_id in 1..934 {
        if !filled_seats.contains(&seat_id)
            && filled_seats.contains(&(seat_id - 1))
            && filled_seats.contains(&(seat_id + 1))
        {
            return seat_id;
        }
    }

    panic!("No empty seat found!");
}
