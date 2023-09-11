use std::{collections::HashSet, hash};

use nom::{bytes::complete::tag, character::complete::line_ending, IResult};

#[derive(Debug)]
struct Bridge {
    head: (i32, i32),
    tail: (i32, i32),
    visited_tail: HashSet<(i32, i32)>,
}

impl Bridge {
    fn new() -> Self {
        let mut visited_tail = HashSet::new();
        visited_tail.insert((0, 0));
        Bridge {
            head: (0, 0),
            tail: (0, 0),
            visited_tail,
        }
    }
    fn apply(&mut self, direction: (i32, i32)) {
        self.head = (self.head.0 + direction.0, self.head.1 + direction.1);
        let mut x = self.head.0 - self.tail.0;
        let mut y = self.head.1 - self.tail.1;
        if x.abs() > 1 || y.abs() > 1 {
            if x.abs() > 0 {
                x /= x.abs();
            }
            if y.abs() > 0 {
                y /= y.abs();
            }
            self.tail = (self.tail.0 + x, self.tail.1 + y);
            self.visited_tail.insert(self.tail);
        }
    }
}

fn main() {
    let mut bridge = Bridge::new();

    let input = std::fs::read_to_string("input.txt").unwrap();
    let (_, result) = parse_lines(input.as_str()).unwrap();
    result
        .iter()
        .map(|(direction, amount)| {
            (0..*amount)
                .map(|_| match direction {
                    Direction::R => (1, 0),
                    Direction::L => (-1, 0),
                    Direction::U => (0, 1),
                    Direction::D => (0, -1),
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .for_each(|direction| {
            bridge.apply(direction);
        });
    println!("count: {}", bridge.visited_tail.len());
}

#[derive(Debug)]
enum Direction {
    R,
    L,
    U,
    D,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    use nom::branch::alt;
    use Direction::*;

    alt((tag("R"), tag("L"), tag("U"), tag("D")))(input).map(|(input, direction)| match direction {
        "R" => (input, R),
        "L" => (input, L),
        "U" => (input, U),
        "D" => (input, D),
        _ => unreachable!(),
    })
}
fn parse_number(input: &str) -> IResult<&str, u32> {
    use nom::character::complete::digit1;
    use nom::combinator::map_res;

    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}
fn parse_line(input: &str) -> IResult<&str, (Direction, u32)> {
    use nom::sequence::tuple;

    let (input, (direction, _, amount)) = tuple((parse_direction, tag(" "), parse_number))(input)?;
    let (input, _) = line_ending(input)?;
    return Ok((input, (direction, amount)));
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    use nom::multi::many1;

    many1(parse_line)(input)
}
