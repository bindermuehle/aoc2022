use core::fmt;
use std::fmt::{Debug, Formatter};

use grid::Grid;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

use crate::Coordinate;

#[derive(Clone)]
pub struct Cave {
    map: Grid<Cell>,
    done: bool,
    min_x: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Rock,
    #[default]
    Air,
    Sand,
}

impl Cave {
    const SAND_SOURCE: Coordinate = Coordinate(500, 0);

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let input = include_str!("input.txt");
        let (_, scan) = parse_scan(&input).unwrap();
        let max = find_max(&scan);
        let mut cave = Cave {
            map: Grid::new(
                (max.1 + 1) as usize,
                (Cave::SAND_SOURCE.0 + max.1 + 1) as usize,
            ),
            done: false,
            min_x: 0,
        };
        cave.add_rocks(scan);
        cave.add_rocky_bottom();
        cave.calculate_min_x();
        cave
    }

    pub fn set_rock(&mut self, x: u32, y: u32) {
        self.map[y as usize][x as usize] = Cell::Rock;
    }
    pub fn is_done(&self) -> bool {
        self.done
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

    pub fn step(&mut self) -> Option<Coordinate> {
        let fall_directions: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];
        let mut x = 500;
        let mut y = 0;
        let mut falling = true;
        while falling {
            for (dx, dy) in fall_directions.iter() {
                let (nx, ny) = (x as i64 + *dx as i64, y as i64 + *dy as i64);
                if nx as usize >= self.map.cols() {
                    self.add_column()
                };
                if nx < 0 {
                    continue;
                }
                match self.map[ny as usize][nx as usize] {
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
        if x == 500 && y == 0 && self.map[y as usize][x as usize] == Cell::Sand {
            self.done = true;
        }
        self.map[y as usize][x as usize] = Cell::Sand;
        return Some(Coordinate(x, y));
    }
    pub fn add_rocky_bottom(&mut self) {
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
    fn calculate_min_x(&mut self) {
        self.min_x = Cave::SAND_SOURCE.0 as u32 - self.map.rows() as u32;
    }
    pub fn get_printable_cells(&self) -> Vec<Vec<Cell>> {
        self.map
            .iter_rows()
            .map(|line| {
                line.skip(self.min_x as usize)
                    .map(|c| c.clone())
                    .collect::<Vec<Cell>>()
            })
            .collect()
    }
}
impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.map.iter_rows() {
            writeln!(
                f,
                "{}",
                line.skip(self.min_x as usize)
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

fn parse_coordinates(input: &str) -> IResult<&str, Vec<Coordinate>> {
    terminated(separated_list1(tag(" -> "), Coordinate::parse), line_ending)(input)
}
pub fn parse_scan(input: &str) -> IResult<&str, Vec<Vec<Coordinate>>> {
    many1(parse_coordinates)(input)
}
pub fn find_max(scan: &Vec<Vec<Coordinate>>) -> Coordinate {
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
