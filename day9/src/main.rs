use nom::{bytes::complete::tag, character::complete::line_ending, IResult};
use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Bridge {
    head: Knot,
    knots: Vec<Knot>,
    visited_tail: HashSet<Knot>,
}

impl Bridge {
    fn new() -> Self {
        let mut visited_tail = HashSet::new();
        visited_tail.insert(Knot::new());
        Bridge {
            head: Knot::new(),
            knots: vec![Knot::new(); 9],
            visited_tail,
        }
    }
    fn apply(&mut self, direction: (i32, i32)) {
        self.head = self.head
            + Knot {
                x: direction.0,
                y: direction.1,
            };
        let mut current = self.head;
        self.knots.iter_mut().for_each(|k| {
            k.follow(current);
            current = *k;
        });
        //self.print();
        self.visited_tail.insert(current.clone());
    }
    #[allow(dead_code)]
    fn print(&self) {
        let mut grid = vec![vec![".".to_string(); 50]; 50];
        println!("{}", vec!["-"; 50].join(""));
        grid[(self.head.y + 25) as usize][(self.head.x + 25) as usize] = "H".to_string();
        self.knots.iter().enumerate().for_each(|(i, k)| {
            grid[(k.y + 25) as usize][(k.x + 25) as usize] = (i + 1).to_string();
        });
        grid.iter().rev().for_each(|row| {
            println!("{}", row.join(""));
        });
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Self {
        Knot { x: 0, y: 0 }
    }
    fn follow(&mut self, other: Self) {
        let mut diff = other - *self;

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            if diff.x.abs() > 0 {
                diff.x /= diff.x.abs();
            }
            if diff.y.abs() > 0 {
                diff.y /= diff.y.abs();
            }
            *self = *self + diff;
        }
    }
}

impl Add for Knot {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Knot {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Knot {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Knot {
            x: self.x - other.x,
            y: self.y - other.y,
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
                    Direction::Right => (1, 0),
                    Direction::Left => (-1, 0),
                    Direction::Up => (0, 1),
                    Direction::Down => (0, -1),
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
    Right,
    Left,
    Up,
    Down,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    use nom::branch::alt;
    use Direction::*;

    alt((tag("R"), tag("L"), tag("U"), tag("D")))(input).map(|(input, direction)| match direction {
        "R" => (input, Right),
        "L" => (input, Left),
        "U" => (input, Up),
        "D" => (input, Down),
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
