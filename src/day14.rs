use aoc_runner_derive::aoc;
use nom::combinator::map;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use nom::{branch::alt, character::complete::digit1};
use nom::{bytes::complete::tag, character::complete::alphanumeric1};
use std::collections::HashMap;

struct BitMask {
    or_mask: u64,
    and_mask: u64,
}

impl BitMask {
    fn new() -> Self {
        Self {
            or_mask: 0,
            and_mask: u64::MAX,
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }
}

impl<'a> From<&'a str> for BitMask {
    fn from(encoded: &'a str) -> Self {
        let mut or_mask = 0;
        let mut and_mask = 0;

        for chr in encoded.chars() {
            and_mask <<= 1;
            or_mask <<= 1;
            match chr {
                'X' => {
                    and_mask |= 1;
                }
                '1' => {
                    and_mask |= 1;
                    or_mask |= 1;
                }
                '0' => {}
                _ => unreachable!(),
            }
        }

        Self { and_mask, or_mask }
    }
}

enum Command<'a> {
    Mask(&'a str),
    Assign(u64, u64),
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(value: &'a str) -> Self {
        alt((mask, assign))(value).unwrap().1
    }
}

fn mask(input: &str) -> IResult<&str, Command<'_>> {
    map(preceded(tag("mask = "), alphanumeric1), Command::Mask)(input)
}

fn assign(input: &str) -> IResult<&str, Command<'_>> {
    map(
        pair(
            delimited(tag("mem["), number, tag("]")),
            preceded(tag(" = "), number),
        ),
        |(location, value)| Command::Assign(location, value),
    )(input)
}

fn number(input: &str) -> IResult<&str, u64> {
    map(digit1, |n: &str| n.parse().unwrap())(input)
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut mask = BitMask::new();
    let mut memory = HashMap::new();

    for cmd in input.lines().map(Command::from) {
        match cmd {
            Command::Mask(new_mask) => {
                mask = new_mask.into();
            }
            Command::Assign(position, value) => {
                memory.insert(position, mask.apply(value));
            }
        }
    }

    memory.values().sum()
}

struct BitMask2 {
    masks: Vec<(u64, u64)>,
}

impl BitMask2 {
    fn new() -> Self {
        Self { masks: Vec::new() }
    }

    fn apply(&self, value: u64) -> impl Iterator<Item = u64> + '_ {
        self.masks
            .iter()
            .map(move |(and_mask, or_mask)| (value & and_mask) | or_mask)
    }
}

impl<'a> From<&'a str> for BitMask2 {
    fn from(value: &'a str) -> Self {
        let mut masks = vec![(0, 0)];

        for chr in value.chars() {
            for value_mut in masks.iter_mut() {
                value_mut.0 <<= 1;
                value_mut.1 <<= 1;
            }

            match chr {
                '0' => {
                    for value_mut in masks.iter_mut() {
                        value_mut.0 |= 1;
                    }
                }
                '1' => {
                    for value_mut in masks.iter_mut() {
                        value_mut.1 |= 1;
                        value_mut.0 |= 1;
                    }
                }
                'X' => {
                    let mut new_masks = Vec::with_capacity(masks.len() * 2);

                    for value in masks.iter() {
                        new_masks.push(*value);
                        new_masks.push((value.0 | 1, value.1 | 1));
                    }

                    masks = new_masks;
                }
                _ => unreachable!(),
            }
        }

        Self { masks }
    }
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut mask = BitMask2::new();
    let mut memory = HashMap::new();

    for cmd in input.lines().map(Command::from) {
        match cmd {
            Command::Mask(new_mask) => mask = new_mask.into(),
            Command::Assign(location, value) => {
                for actual in mask.apply(location) {
                    memory.insert(actual, value);
                }
            }
        }
    }

    memory.values().sum()
}
