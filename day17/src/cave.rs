use std::{iter::Cycle, vec::IntoIter};

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Block,
}
#[derive(Clone, PartialEq)]
pub struct Coordinate((u32, u32));

#[derive(PartialEq, Debug, Clone)]
struct Tile {
    x: u32,
    y: u32,
    kind: TileType,
}

#[derive(Clone)]
struct Shape {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    coordinates: Vec<Coordinate>,
}

impl Shape {
    fn collides_with(&self, other: &Shape) -> bool {
        let self_coordinates = self.get_coordinates();
        let other_coordinates = other.get_coordinates();
        self_coordinates
            .iter()
            .any(|c| other_coordinates.contains(c))
    }
    fn get_coordinates(&self) -> Vec<Coordinate> {
        self.coordinates
            .iter()
            .map(|Coordinate((x, y))| Coordinate((x + self.x, y + self.y)))
            .collect()
    }
}
#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Option<Direction> {
        match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => unreachable!("Invalid direction"),
        }
    }
}
pub struct Cave {
    pub width: u32,
    pub height: u32,
    falling_shape: Option<Shape>,
    stationary_shapes: Vec<Shape>,
    steps: u32,
    shape_iterator: Cycle<IntoIter<Shape>>,
    direction_iterator: Cycle<IntoIter<Direction>>,
    down: bool,
}

impl Cave {
    const FALLING_SHAPE_X: u32 = 2;
    const FREE_SPACE: u32 = 3;
    const EMPTY_ROWS: u32 = 3;
    const WIDTH: u32 = 7;
    pub fn new() -> Cave {
        let shape_iterator = create_shapes().into_iter().cycle();
        let direction_iterator = parse_directions().into_iter().cycle();
        Cave {
            width: Self::WIDTH,
            height: Self::EMPTY_ROWS,
            falling_shape: None,
            stationary_shapes: vec![],
            steps: 0,
            shape_iterator,
            direction_iterator,
            down: false,
        }
    }
    pub fn is_done(&self) -> bool {
        self.stationary_shapes.len() == 2022
    }
    pub fn get_cells(&self) -> Vec<TileType> {
        let mut cells = vec![TileType::Empty; (self.width * (self.height + 1)) as usize];
        if let Some(shape) = self.falling_shape.as_ref() {
            shape
                .get_coordinates()
                .iter()
                .for_each(|Coordinate((x, y))| {
                    cells[(y * self.width + x) as usize] = TileType::Block
                });
        };
        self.stationary_shapes
            .iter()
            .flat_map(|shape| shape.get_coordinates())
            .for_each(|Coordinate((x, y))| cells[(y * self.width + x) as usize] = TileType::Block);
        return cells;
    }
    pub fn step(&mut self) {
        if self.falling_shape.is_none() {
            self.add_falling_shape();
        } else {
            let shape = self.falling_shape.take().unwrap();
            if self.down {
                let new_shape = self.move_element(0, -1, shape.clone());
                if let Some(new_shape) = new_shape {
                    self.falling_shape = Some(new_shape);
                } else {
                    self.falling_shape = None;
                    self.stationary_shapes.push(shape);
                }
                self.down = false;
            } else {
                let direction = self.direction_iterator.next().unwrap();
                let new_shape = match direction {
                    Direction::Left => self.move_element(-1, 0, shape.clone()),
                    Direction::Right => self.move_element(1, 0, shape.clone()),
                };
                if let Some(new_shape) = new_shape {
                    self.falling_shape = Some(new_shape);
                } else {
                    self.falling_shape = Some(shape);
                }
                self.down = true;
            }
        }
        self.steps += 1;
    }

    pub fn add_falling_shape(&mut self) {
        let shape = self.shape_iterator.next().unwrap().clone();
        let x = Self::FALLING_SHAPE_X;
        let y = self.get_highest_point() + Self::FREE_SPACE;
        if y + shape.height > self.height {
            self.height = y + shape.height;
        }
        self.falling_shape = Some(Shape { x, y, ..shape });
    }
    pub fn get_highest_point(&self) -> u32 {
        self.stationary_shapes
            .iter()
            .map(|shape| shape.y + shape.height)
            .max()
            .unwrap_or(0)
    }
    fn move_element(&self, x: i32, y: i32, shape: Shape) -> Option<Shape> {
        let x = shape.x.checked_add_signed(x);
        let y = shape.y.checked_add_signed(y);
        if let (Some(x), Some(y)) = (x, y) {
            if x + shape.width > self.width {
                return None;
            }

            let new_shape = Shape { x, y, ..shape };
            for shape in self.stationary_shapes.iter() {
                if new_shape.collides_with(shape) {
                    return None;
                }
            }
            return Some(new_shape);
        }
        None
    }
}
fn parse_directions() -> Vec<Direction> {
    let input = include_str!("input.txt");
    input.chars().filter_map(Direction::parse).collect()
}
fn create_shapes() -> Vec<Shape> {
    vec![
        Shape {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
            coordinates: vec![
                Coordinate((0, 0)),
                Coordinate((1, 0)),
                Coordinate((2, 0)),
                Coordinate((3, 0)),
            ],
        },
        Shape {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
            coordinates: vec![
                Coordinate((1, 0)),
                Coordinate((0, 1)),
                Coordinate((1, 1)),
                Coordinate((1, 2)),
                Coordinate((2, 1)),
            ],
        },
        Shape {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
            coordinates: vec![
                Coordinate((2, 2)),
                Coordinate((2, 1)),
                Coordinate((0, 0)),
                Coordinate((1, 0)),
                Coordinate((2, 0)),
            ],
        },
        Shape {
            x: 0,
            y: 0,
            width: 1,
            height: 4,
            coordinates: vec![
                Coordinate((0, 0)),
                Coordinate((0, 1)),
                Coordinate((0, 2)),
                Coordinate((0, 3)),
            ],
        },
        Shape {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
            coordinates: vec![
                Coordinate((0, 0)),
                Coordinate((1, 0)),
                Coordinate((0, 1)),
                Coordinate((1, 1)),
            ],
        },
    ]
}
