use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

#[derive(Debug)]
pub enum Expr {
    Value(u64),
    Sum(Box<Expr>, Box<Expr>),
    Product(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn value(&self) -> u64 {
        use Expr::*;

        match self {
            Value(num) => *num,
            Sum(lhs, rhs) => lhs.value() + rhs.value(),
            Product(lhs, rhs) => lhs.value() * rhs.value(),
        }
    }
}

enum Op {
    Add,
    Multiply,
}

fn expression(input: &str) -> IResult<&str, Expr> {
    let (input, operand) = term(input)?;
    let (input, rest) = many0(pair(operator, term))(input)?;

    let expr = rest.into_iter().fold(operand, |lhs, (op, rhs)| match op {
        Op::Add => Expr::Sum(Box::new(lhs), Box::new(rhs)),
        Op::Multiply => Expr::Product(Box::new(lhs), Box::new(rhs)),
    });

    Ok((input, expr))
}

fn operator(input: &str) -> IResult<&str, Op> {
    alt((
        map(tag(" + "), |_| Op::Add),
        map(tag(" * "), |_| Op::Multiply),
    ))(input)
}

fn term(input: &str) -> IResult<&str, Expr> {
    alt((number, parenthized))(input)
}

fn parenthized(input: &str) -> IResult<&str, Expr> {
    delimited(tag("("), expression, tag(")"))(input)
}

fn expression_2(input: &str) -> IResult<&str, Expr> {
    let (input, operand) = factor_2(input)?;
    let (input, rest) = many0(preceded(tag(" * "), factor_2))(input)?;

    let expr = rest.into_iter().fold(operand, |lhs, rhs| {
        Expr::Product(Box::new(lhs), Box::new(rhs))
    });

    Ok((input, expr))
}

fn factor_2(input: &str) -> IResult<&str, Expr> {
    let (input, operand) = term_2(input)?;
    let (input, rest) = many0(preceded(tag(" + "), term_2))(input)?;

    let expr = rest
        .into_iter()
        .fold(operand, |lhs, rhs| Expr::Sum(Box::new(lhs), Box::new(rhs)));

    Ok((input, expr))
}

fn term_2(input: &str) -> IResult<&str, Expr> {
    alt((number, parenthized_2))(input)
}

fn parenthized_2(input: &str) -> IResult<&str, Expr> {
    delimited(tag("("), expression_2, tag(")"))(input)
}

fn number(input: &str) -> IResult<&str, Expr> {
    map(digit1, |value: &str| Expr::Value(value.parse().unwrap()))(input)
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| all_consuming(expression)(line).unwrap().1.value())
        .sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| all_consuming(expression_2)(line).unwrap().1.value())
        .sum()
}
