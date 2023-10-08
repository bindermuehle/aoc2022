use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::i32,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

const ROW: i32 = 2000000;

#[derive(Debug)]
struct Sensor {
    coordinates: (i32, i32),
    beacon: Option<(i32, i32)>,
}
impl Sensor {
    fn new(coordinates: (i32, i32), beacon: Option<(i32, i32)>) -> Self {
        Self {
            coordinates,
            beacon,
        }
    }
    fn parse(input: &str) -> IResult<&str, Sensor> {
        let (input, (s, b)) = separated_pair(
            delimited(tag("Sensor at "), parse_coordinates, tag(": ")),
            tag("closest beacon is at "),
            parse_coordinates,
        )(input)?;
        Ok((input, Self::new(s, Some(b))))
    }
    fn distance(&self, other: &(i32, i32)) -> i32 {
        (self.coordinates.0 - other.0).abs() + (self.coordinates.1 - other.1).abs()
    }
    fn beacon_distance(&self) -> i32 {
        self.distance(&self.beacon.unwrap())
    }
    fn is_closer(&self, other: &(i32, i32)) -> bool {
        self.distance(other) <= self.beacon_distance()
    }
}

fn main() {
    let sensors: Vec<Sensor> = std::fs::read_to_string("input.txt")
        .expect("input.txt not found")
        .lines()
        .map(|line| Sensor::parse(line).unwrap().1)
        .collect();
    let mut row = HashSet::new();
    sensors.iter().for_each(|s| {
        let mut x = s.coordinates.0;
        let y = ROW;
        while s.is_closer(&(x, y)) {
            if s.beacon != Some((x, y)) {
                row.insert(x);
            }
            x += 1;
        }
        x = s.coordinates.0 - 1;
        while s.is_closer(&(x, y)) {
            if s.beacon != Some((x, y)) {
                row.insert(x);
            }
            x -= 1;
        }
    });
    let mut v: Vec<&i32> = row.iter().collect();
    v.sort();
    println!("{:?}", v);
    println!("{}", row.len());
}

fn parse_coordinates(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(preceded(tag("x="), i32), tag(", y="), i32)(input)
}
