use std::cmp::Ordering;

use nom::branch::alt;
use nom::character::complete::{char, line_ending, u32};

use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, terminated, tuple};
use nom::{multi::separated_list0, IResult};

#[derive(Debug, PartialEq, Eq)]
enum Unit {
    Number(u32),
    List(Vec<Unit>),
}

impl Unit {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(u32, |n| Unit::Number(n)),
            map(parse_unit_list, |l| Unit::List(l)),
        ))(input)
    }
    fn with_slice<T>(&self, f: impl FnOnce(&[Unit]) -> T) -> T {
        match self {
            Unit::Number(n) => f(&vec![Unit::Number(*n)]),
            Unit::List(l) => f(&l[..]),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (_, signal) = parse_distress_signal(&input).unwrap();
    let count: usize = signal
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
            if a < b {
                return i + 1;
            }
            0
        })
        .sum();
    println!("part 1: {}", count);
}

fn parse_unit_list(input: &str) -> IResult<&str, Vec<Unit>> {
    delimited(
        char('['),
        separated_list0(char(','), Unit::parse),
        char(']'),
    )(input)
}

fn parse_unit_tuple(input: &str) -> IResult<&str, (Unit, Unit)> {
    tuple((
        map(terminated(parse_unit_list, line_ending), |l| Unit::List(l)),
        map(terminated(parse_unit_list, line_ending), |l| Unit::List(l)),
    ))(input)
}
fn parse_distress_signal(input: &str) -> IResult<&str, Vec<(Unit, Unit)>> {
    separated_list1(line_ending, parse_unit_tuple)(input)
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Unit::Number(l), Unit::Number(r)) => l.partial_cmp(r),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(l, r)| l.cmp(r))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_number_list() {
        assert_eq!(
            parse_unit_list("[1,2,3,4,5]"),
            Ok((
                "",
                vec![
                    Unit::Number(1),
                    Unit::Number(2),
                    Unit::Number(3),
                    Unit::Number(4),
                    Unit::Number(5)
                ]
            ))
        );
    }
    #[test]
    fn test_parse_embedded_list() {
        assert_eq!(
            parse_unit_list("[[1,2],3,4,5]"),
            Ok((
                "",
                vec![
                    Unit::List(vec![Unit::Number(1), Unit::Number(2),]),
                    Unit::Number(3),
                    Unit::Number(4),
                    Unit::Number(5)
                ]
            ))
        );
    }
    #[test]
    fn test_parse_separated_tuple() {
        assert_eq!(
            parse_unit_tuple("[[1],[2,3,4]]\n[[1],4]\n"),
            Ok((
                "",
                (
                    Unit::List(vec![
                        Unit::List(vec![(Unit::Number(1))]),
                        Unit::List(vec![Unit::Number(2), Unit::Number(3), Unit::Number(4)])
                    ]),
                    Unit::List(vec![Unit::List(vec![Unit::Number(1)]), Unit::Number(4)])
                )
            ))
        )
    }
}
