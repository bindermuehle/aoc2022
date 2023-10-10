use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::i64,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::{collections::HashSet, ops::RangeInclusive};

struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let sensors = input
            .lines()
            .map(|line| Sensor::parse(line).unwrap().1)
            .collect();
        Map { sensors }
    }
    fn find_ranges(&self, y: i64) -> impl Iterator<Item = RangeInclusive<i64>> {
        let mut ranges = self.sensors.iter().fold(vec![], |mut a, sensor| {
            let x = sensor.coordinate.0;
            let distance = sensor.beacon_distance() - sensor.distance(&(x, y));
            if distance > 0 {
                let start = x - distance;
                let end = x + distance;
                a.push(start..=end);
            }
            a
        });
        ranges.sort_by(|a, b| a.start().cmp(b.start()));
        ranges.into_iter().coalesce(|a, b| {
            if a.end() + 1 >= *b.start() {
                if a.end() < b.end() {
                    Ok(*a.start()..=*b.end())
                } else {
                    Ok(a)
                }
            } else {
                Err((a, b))
            }
        })
    }
    fn find_impossible_beacon_positions(&self, y: i64) -> i64 {
        let beacons: HashSet<i64> = self
            .sensors
            .iter()
            .filter(|s| s.beacon.unwrap().1 == y)
            .map(|s| s.beacon.unwrap().0)
            .collect();

        self.find_ranges(y)
            .map(|r| {
                let size = r.end() - r.start() + 1;
                let becaons_in_range = beacons.iter().filter(|b| r.contains(b)).count();
                size - becaons_in_range as i64
            })
            .sum::<i64>()
    }
    fn find_space_in_row(&self, y: i64, range: RangeInclusive<i64>) -> Option<Coordinate> {
        let ranges: Vec<RangeInclusive<i64>> = self.find_ranges(y).collect();
        if ranges.len() > 1 {
            for r in ranges.iter() {
                if range.contains(&(r.end() + 1)) {
                    return Some((r.end() + 1, y));
                }
            }
        }
        return None;
    }
}

type Coordinate = (i64, i64);
#[derive(Debug)]
struct Sensor {
    coordinate: Coordinate,
    beacon: Option<Coordinate>,
}
impl Sensor {
    fn new(coordinates: (i64, i64), beacon: Option<(i64, i64)>) -> Self {
        Self {
            coordinate: coordinates,
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
    fn distance(&self, other: &(i64, i64)) -> i64 {
        (self.coordinate.0 - other.0).abs() + (self.coordinate.1 - other.1).abs()
    }
    fn beacon_distance(&self) -> i64 {
        self.distance(&self.beacon.unwrap())
    }
}

fn main() {
    let map = Map::parse(std::fs::read_to_string("input.txt").unwrap().as_str());
    //    println!("{}", map.find_impossible_beacon_positions(2000000));
    let max = 4_000_000;
    let range = 0..=max;
    for y in range {
        if let Some((x, y)) = map.find_space_in_row(y, 0..=max) {
            println!("{} {}", x, y);
            println!("{}", x * max + y);
            break;
        }
    }
}

fn parse_coordinates(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(preceded(tag("x="), i64), tag(", y="), i64)(input)
}
