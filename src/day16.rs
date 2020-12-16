use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::{map, recognize};
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

type ValidRange = RangeInclusive<u32>;

fn field_name(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alpha1, space1))))(input)
}

fn range(input: &str) -> IResult<&str, ValidRange> {
    map(
        separated_pair(digit1, tag("-"), digit1),
        |(start, end): (&str, &str)| start.parse().unwrap()..=end.parse().unwrap(),
    )(input)
}

fn allowed_values(input: &str) -> IResult<&str, (ValidRange, ValidRange)> {
    separated_pair(range, tag(" or "), range)(input)
}

fn ticket_field(input: &str) -> IResult<&str, (&str, (ValidRange, ValidRange))> {
    separated_pair(field_name, tag(": "), allowed_values)(input)
}

struct Requirements<'a>(HashMap<&'a str, (ValidRange, ValidRange)>);

impl<'a> Requirements<'a> {
    fn valid_ranges(&self, value: u32) -> HashSet<&'a str> {
        self.0
            .iter()
            .filter(|(_, (low, high))| low.contains(&value) || high.contains(&value))
            .map(|(name, _)| *name)
            .collect()
    }
}

struct Tickets {
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Tickets {
    fn valid_ranges_for_position<'a>(
        &self,
        requirements: &Requirements<'a>,
        position: usize,
    ) -> HashSet<&'a str> {
        self.nearby_tickets
            .iter()
            .map(|ticket| requirements.valid_ranges(ticket[position]))
            .fold(
                requirements.valid_ranges(self.my_ticket[position]),
                |set, curr| set.intersection(&curr).copied().collect(),
            )
    }
}

fn parse_input(input: &str) -> (Requirements<'_>, Tickets) {
    let mut parts = input.split("\n\n");

    let requirements = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| ticket_field(l).unwrap().1)
        .collect();

    let my_ticket = parts
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby_tickets = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (
        Requirements(requirements),
        Tickets {
            my_ticket,
            nearby_tickets,
        },
    )
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (requirements, tickets) = parse_input(input);
    tickets
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|n| requirements.valid_ranges(**n).is_empty())
        .sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let (requirements, mut tickets) = parse_input(input);
    tickets.nearby_tickets = tickets
        .nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|n| !requirements.valid_ranges(*n).is_empty())
        })
        .collect();

    let ranges: Vec<_> =
        (0..20) // 20 is the number of fields
            .map(|index| tickets.valid_ranges_for_position(&requirements, index))
            .collect();

    let solved = solve_requirements(0, &ranges, HashSet::new()).unwrap();
    // Note: The solution is in reverse order since we push onto the end

    solved
        .into_iter()
        .enumerate()
        .filter_map(|(index, key)| {
            if key.starts_with("departure") {
                Some(tickets.my_ticket[19 - index] as u64)
            } else {
                None
            }
        })
        .product()
}

fn solve_requirements<'a>(
    start: usize,
    sets: &[HashSet<&'a str>],
    removed: HashSet<&'a str>,
) -> Option<Vec<&'a str>> {
    match sets.get(start) {
        Some(set) => {
            for key in set.difference(&removed) {
                let mut new_removed = removed.clone();
                new_removed.insert(key);

                if let Some(mut found) = solve_requirements(start + 1, sets, new_removed) {
                    found.push(key);
                    return Some(found);
                }
            }
            None
        }
        None => Some(Vec::new()),
    }
}
