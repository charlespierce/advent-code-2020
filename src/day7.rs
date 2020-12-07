use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, line_ending};
use nom::combinator::{all_consuming, eof, map, recognize};
use nom::multi::{fold_many1, many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;
use std::collections::{HashMap, HashSet, VecDeque};

type Contained<'a> = (u32, &'a str);
type Bag<'a> = (&'a str, Vec<Contained<'a>>);

fn parse_part1(input: &str) -> HashMap<&str, HashSet<&str>> {
    all_consuming(fold_many1(
        bag,
        HashMap::new(),
        |mut map, (container, contained)| {
            for (_, name) in contained {
                map.entry(name)
                    .or_insert_with(HashSet::new)
                    .insert(container);
            }
            map
        },
    ))(input)
    .unwrap()
    .1
}

pub fn parse_part2(input: &str) -> HashMap<&str, Vec<Contained<'_>>> {
    all_consuming(many1(bag))(input)
        .unwrap()
        .1
        .into_iter()
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut containers = parse_part1(input);
    let mut results = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");

    while let Some(bag) = queue.pop_front() {
        if let Some(outer_bags) = containers.remove(&bag) {
            for outer in outer_bags {
                results.insert(outer);
                queue.push_back(outer);
            }
        }
    }

    results.len()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let containers = parse_part2(input);
    let mut cache = HashMap::new();
    bags_in("shiny gold", &containers, &mut cache) - 1
}

fn bags_in(
    name: &str,
    containers: &HashMap<&str, Vec<Contained<'_>>>,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(value) = cache.get(name) {
        return *value;
    }

    let mut total = 1;
    for (count, inner) in containers.get(name).unwrap() {
        total += *count * bags_in(*inner, &containers, cache);
    }

    cache.insert(name.into(), total);

    total
}

fn bag_name(input: &str) -> IResult<&str, &str> {
    recognize(tuple((alpha1, char(' '), alpha1)))(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map(digit1, |n: &str| n.parse().unwrap())(input)
}

fn single_contained(input: &str) -> IResult<&str, Contained<'_>> {
    map(delimited(tag("1 "), bag_name, tag(" bag")), |name| {
        (1, name)
    })(input)
}

fn multiple_contained(input: &str) -> IResult<&str, Contained<'_>> {
    terminated(separated_pair(number, char(' '), bag_name), tag(" bags"))(input)
}

fn eol(input: &str) -> IResult<&str, &str> {
    terminated(tag("."), alt((line_ending, eof)))(input)
}

fn contained_list(input: &str) -> IResult<&str, Vec<Contained<'_>>> {
    terminated(
        alt((
            map(tag("no other bags"), |_| Vec::new()),
            separated_list1(tag(", "), alt((single_contained, multiple_contained))),
        )),
        eol,
    )(input)
}

fn bag(input: &str) -> IResult<&str, Bag<'_>> {
    separated_pair(bag_name, tag(" bags contain "), contained_list)(input)
}
