use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, space0};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use std::collections::HashMap;
use std::iter::{empty, once};

#[derive(Debug)]
enum Rule {
    Char(char),
    Sequence(Vec<usize>),
    Alt(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn parse<'a>(&self, input: &'a str, rules: &HashMap<usize, Rule>) -> Option<&'a str> {
        match self {
            Rule::Char(chr) => {
                if input.starts_with(*chr) {
                    Some(&input[1..])
                } else {
                    None
                }
            }
            Rule::Sequence(others) => others.iter().try_fold(input, |input, index| {
                rules.get(index).unwrap().parse(input, rules)
            }),
            Rule::Alt(first, second) => first
                .parse(input, rules)
                .or_else(|| second.parse(input, rules)),
        }
    }

    fn parse_all<'a>(
        &'a self,
        input: &'a str,
        rules: &'a HashMap<usize, Rule>,
    ) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        match self {
            Rule::Char(chr) => {
                if input.starts_with(*chr) {
                    Box::new(once(&input[1..]))
                } else {
                    Box::new(empty())
                }
            }
            Rule::Alt(first, second) => Box::new(
                first
                    .parse_all(input, rules)
                    .chain(second.parse_all(input, rules)),
            ),
            Rule::Sequence(others) => {
                others
                    .iter()
                    .fold(Box::new(once(input)), move |inputs, index| {
                        Box::new(
                            inputs
                                .map(move |i| rules.get(index).unwrap().parse_all(i, rules))
                                .flatten(),
                        )
                    })
            }
        }
    }
}

fn rule_line(input: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(index, tag(": "), rule)(input)
}

fn index(input: &str) -> IResult<&str, usize> {
    map(digit1, |n: &str| n.parse().unwrap())(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    alt((char_rule, reference_rule))(input)
}

fn reference_rule(input: &str) -> IResult<&str, Rule> {
    let (input, first) = sequence_rule(input)?;
    match preceded(tag(" | "), sequence_rule)(input) {
        Ok((output, second)) => Ok((output, Rule::Alt(Box::new(first), Box::new(second)))),
        Err(_) => Ok((input, first)),
    }
}

fn sequence_rule(input: &str) -> IResult<&str, Rule> {
    map(many1(preceded(space0, index)), Rule::Sequence)(input)
}

fn char_rule(input: &str) -> IResult<&str, Rule> {
    map(delimited(tag("\""), anychar, tag("\"")), Rule::Char)(input)
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    input.lines().map(|l| rule_line(l).unwrap().1).collect()
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rules = parse_rules(parts.next().unwrap());
    let rule_0 = rules.get(&0).unwrap();

    let messages = parts.next().unwrap();

    messages
        .lines()
        .filter(|l| matches!(rule_0.parse(l, &rules), Some("")))
        .count()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rules = parse_rules(
        &parts
            .next()
            .unwrap()
            .replace("8: 42", "8: 42 | 42 8")
            .replace("11: 42 31", "11: 42 31 | 42 11 31"),
    );
    let rule_0 = rules.get(&0).unwrap();

    let messages = parts.next().unwrap();

    messages
        .lines()
        .filter(|l| rule_0.parse_all(l, &rules).any(|left| left == ""))
        .count()
}
