use core::fmt;
use std::fmt::{Debug, Formatter};

use grid::Grid;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    #[default]
    Air,
    Sand,
}
#[derive(Debug)]
struct Coordinate(u32, u32);

impl Coordinate {
    fn parse(input: &str) -> IResult<&str, Coordinate> {
        map(separated_pair(u32, tag(","), u32), |(x, y)| {
            Coordinate(x, y)
        })(input)
    }
}
#[derive(Clone)]
struct Cave {
    map: Grid<Cell>,
}
struct SandIter<'a>(&'a mut Cave);
impl SandIter<'_> {
    const FALL_DIRECTIONS: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];
}
impl Iterator for SandIter<'_> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = 500;
        let mut y = 0;
        let mut falling = true;
        while falling {
            for (dx, dy) in Self::FALL_DIRECTIONS.iter() {
                let (nx, ny) = (x as i64 + *dx as i64, y as i64 + *dy as i64);
                if nx as usize >= self.0.map.cols() {
                    self.0.add_column()
                };
                if nx < 0 {
                    continue;
                }
                match self.0.map[ny as usize][nx as usize] {
                    Cell::Air => {
                        falling = true;
                        x = nx as u32;
                        y = ny as u32;
                        break;
                    }
                    Cell::Rock | Cell::Sand => falling = false,
                }
            }
        }
        // println!("{:?}", self.0);
        if x == 500 && y == 0 && self.0.map[y as usize][x as usize] == Cell::Sand {
            return None;
        }
        self.0.map[y as usize][x as usize] = Cell::Sand;
        return Some(Coordinate(x, y));
    }
}
impl<'a> Cave {
    fn new(max: Coordinate) -> Self {
        Cave {
            map: Grid::new((max.1 + 1) as usize, (max.0 + 1) as usize),
        }
    }
    fn set_rock(&mut self, x: u32, y: u32) {
        self.map[y as usize][x as usize] = Cell::Rock;
    }
    fn add_rocks(&mut self, scan: Vec<Vec<Coordinate>>) {
        scan.iter().for_each(|l| {
            l.iter().enumerate().for_each(|(i, c)| {
                if i == l.len() - 1 {
                    return;
                }
                let min = std::cmp::min(c.0, l[i + 1].0);
                let max = std::cmp::max(c.0, l[i + 1].0);
                (min..=max).for_each(|x| {
                    let min = std::cmp::min(c.1, l[i + 1].1);
                    let max = std::cmp::max(c.1, l[i + 1].1);
                    (min..=max).for_each(|y| self.set_rock(x, y))
                });
            })
        });
    }
    fn emulate_sand(&mut self) -> usize {
        self.pour_sand().count()
    }

    fn pour_sand(&'a mut self) -> SandIter<'a> {
        SandIter(self)
    }
    fn add_rocky_bottom(&mut self) {
        self.add_row(Cell::Air);
        self.add_row(Cell::Rock);
    }
    fn add_row(&mut self, cell: Cell) {
        self.map
            .insert_row(self.map.rows(), vec![cell; self.map.cols()]);
    }
    fn add_column(&mut self) {
        self.map
            .insert_col(self.map.cols(), vec![Cell::Air; self.map.rows()]);
        let x = self.map.rows() - 1;
        let y = self.map.cols() - 1;
        self.map[x][y] = Cell::Rock;
    }
    fn find_minx_sand(&self) -> u32 {
        self.map
            .indexed_iter()
            .fold(u32::MAX, |acc, ((_, c), cell)| match cell {
                Cell::Sand => std::cmp::min(acc, c as u32),
                _ => acc,
            })
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let min = self.find_minx_sand();
        for line in self.map.iter_rows() {
            writeln!(
                f,
                "{}",
                line.skip(min as usize)
                    .map(|c| match c {
                        Cell::Rock => '#',
                        Cell::Air => '.',
                        Cell::Sand => 'o',
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let scan = parse_scan(&input).unwrap().1;
    let mut cave = Cave::new(find_max(&scan));
    cave.add_rocks(scan);
    cave.add_rocky_bottom();
    println!("count: {}", cave.emulate_sand());
    println!("{:?}", cave);
}
fn parse_coordinates(input: &str) -> IResult<&str, Vec<Coordinate>> {
    terminated(separated_list1(tag(" -> "), Coordinate::parse), line_ending)(input)
}
fn parse_scan(input: &str) -> IResult<&str, Vec<Vec<Coordinate>>> {
    many1(parse_coordinates)(input)
}
fn find_max(scan: &Vec<Vec<Coordinate>>) -> Coordinate {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in scan {
        for coord in line {
            if coord.0 > max_x {
                max_x = coord.0;
            }
            if coord.1 > max_y {
                max_y = coord.1;
            }
        }
    }
    Coordinate(max_x, max_y)
}
