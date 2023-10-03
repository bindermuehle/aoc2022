use core::fmt;
use grid::Grid;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::fmt::{Debug, Formatter};
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsCast};
use web_sys::ImageData;
#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    #[default]
    Air,
    Sand,
}
#[derive(Debug)]
#[wasm_bindgen]
pub struct Coordinate(u32, u32);

#[wasm_bindgen]
impl Coordinate {
    fn parse(input: &str) -> IResult<&str, Coordinate> {
        map(separated_pair(u32, tag(","), u32), |(x, y)| {
            Coordinate(x, y)
        })(input)
    }
}
#[derive(Clone)]
#[wasm_bindgen]
pub struct Cave {
    map: Grid<Cell>,
    done: bool,
    min_x: u32,
}

#[wasm_bindgen]
impl Cave {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let input = include_str!("input.txt");
        let (_, scan) = parse_scan(&input).unwrap();
        let max = find_max(&scan);
        let mut cave = Cave {
            map: Grid::new((max.1 + 1) as usize, (max.0 + 1) as usize),
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
    #[wasm_bindgen]
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
        self.min_x = self
            .map
            .indexed_iter()
            .fold(u32::MAX, |acc, ((r, c), cell)| match cell {
                Cell::Air => acc,
                _ => {
                    if r < self.map.rows() - 2 {
                        std::cmp::min(acc, c as u32)
                    } else {
                        acc
                    }
                }
            })
    }
    #[wasm_bindgen]
    pub fn render(&mut self, canvas_id: &str) {
        self.calculate_min_x();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        canvas.set_width(self.map.cols() as u32 - self.min_x);
        canvas.set_height(self.map.rows() as u32);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context
            .put_image_data(&self.draw_image(), 0.0, 0.0)
            .unwrap();
    }

    fn draw_image(&self) -> ImageData {
        let width = self.map.cols() - self.min_x as usize;
        // let air_color = JsValue::from_str("#4db4e3");
        let air_color: [u8; 4] = [77, 180, 227, 255];
        // let rock_color = JsValue::from_str("#33302d");
        let rock_color: [u8; 4] = [51, 48, 45, 255];
        // let sand_color = JsValue::from_str("#827f58");
        let sand_color: [u8; 4] = [130, 127, 88, 255];

        let pixels: Vec<u8> = (0..self.map.rows())
            .flat_map(|y| {
                let test = (self.min_x as usize..self.map.cols())
                    .flat_map(|x| match self.map[y][x] {
                        Cell::Air => air_color.to_vec(),
                        Cell::Rock => rock_color.to_vec(),
                        Cell::Sand => sand_color.to_vec(),
                    })
                    .collect::<Vec<u8>>();
                return test;
            })
            .collect();
        ImageData::new_with_u8_clamped_array(Clamped(&pixels[..]), width as _).unwrap()
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
