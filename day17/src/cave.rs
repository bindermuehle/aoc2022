use std::{collections::HashMap, iter::Cycle, iter::Enumerate, vec::IntoIter};

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Block,
}
#[derive(Clone, PartialEq)]
pub struct Coordinate((u64, u64));

#[derive(PartialEq, Debug, Clone)]
struct Tile {
    x: u64,
    y: u64,
    kind: TileType,
}

#[derive(Clone)]
struct Shape {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
    coordinates: Vec<Coordinate>,
    kind: ShapeKind,
}
#[derive(Clone, PartialEq)]
enum ShapeKind {
    Horizontal,
    Vertical,
    Square,
    LShape,
    PlusShape,
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
    pub width: u64,
    pub height: u64,
    falling_shape: Option<Shape>,
    stationary_shapes: Vec<Shape>,
    steps: u64,
    shape_iterator: Cycle<IntoIter<Shape>>,
    direction_iterator: Cycle<Enumerate<IntoIter<Direction>>>,
    direction_index: usize,
    down: bool,
    minimum_repetition_size: u64,
    unique_position: HashMap<(usize, u64), usize>,
    done: bool,
}

impl Cave {
    const FALLING_SHAPE_X: u64 = 2;
    const FREE_SPACE: u64 = 3;
    const EMPTY_ROWS: u64 = 3;
    const WIDTH: u64 = 7;
    //const AMOUNT: usize = 2022;
    const AMOUNT: usize = 1000000000000;
    pub fn new() -> Cave {
        let shape_iterator = create_shapes();
        let direction_iterator = parse_directions();

        Cave {
            width: Self::WIDTH,
            height: Self::EMPTY_ROWS,
            falling_shape: None,
            stationary_shapes: vec![],
            steps: 0,
            minimum_repetition_size: (shape_iterator.len() * direction_iterator.len()) as u64,
            shape_iterator: shape_iterator.into_iter().cycle(),
            direction_iterator: direction_iterator.into_iter().enumerate().cycle(),
            direction_index: 0,
            down: false,
            unique_position: HashMap::new(),
            done: false,
        }
    }
    pub fn is_done(&self) -> bool {
        self.done
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
                    self.find_repetition();
                    self.unique_position.insert(
                        (
                            self.direction_index,
                            self.stationary_shapes.last().unwrap().x,
                        ),
                        self.stationary_shapes.len() - 1,
                    );
                }
                self.down = false;
            } else {
                let (index, direction) = self.direction_iterator.next().unwrap();
                let new_shape = match direction {
                    Direction::Left => self.move_element(-1, 0, shape.clone()),
                    Direction::Right => self.move_element(1, 0, shape.clone()),
                };
                self.direction_index = index;
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
    pub fn get_highest_point(&self) -> u64 {
        self.stationary_shapes
            .iter()
            .map(|shape| shape.y + shape.height)
            .max()
            .unwrap_or(0)
    }
    fn move_element(&self, x: i64, y: i64, shape: Shape) -> Option<Shape> {
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
    fn find_repetition(&mut self) -> bool {
        if let Some(index) = self.unique_position.get(&(
            self.direction_index,
            self.stationary_shapes.last().unwrap().x,
        )) {
            if self.stationary_shapes[*index].kind == self.stationary_shapes.last().unwrap().kind {
                for i in 0..self.stationary_shapes.len() as usize - index {
                    let a = &self.stationary_shapes[(index - i) as usize];
                    let b = &self.stationary_shapes
                        [(self.stationary_shapes.len() - 1 - i as usize) as usize];
                    assert!(a.kind == b.kind);
                    if a.x != b.x {
                        return false;
                    }
                }
                println!(
                    "Found repetition at {} and {}",
                    self.stationary_shapes.len(),
                    index
                );
                let highest_point = self.get_highest_point();
                let repetition_length = self.stationary_shapes.len() - 1 - index;
                let highest_point_index = self.stationary_shapes[0..=*index]
                    .iter()
                    .map(|s| s.y + s.height)
                    .max()
                    .unwrap();
                let max_repetition = highest_point - highest_point_index;
                let missing = Self::AMOUNT - (self.stationary_shapes.len());
                let mut result = highest_point;
                result += missing as u64 / repetition_length as u64 * max_repetition;
                result += self.stationary_shapes
                    [*index..=*index + (missing as usize % repetition_length)]
                    .iter()
                    .map(|s| s.y + s.height)
                    .max()
                    .unwrap()
                    - highest_point_index;
                println!("Heighest point: {}", result);
                self.done = true;
                return true;
            }
        }
        false
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
            kind: ShapeKind::Horizontal,
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
            kind: ShapeKind::PlusShape,
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
            kind: ShapeKind::LShape,
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
            kind: ShapeKind::Vertical,
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
            kind: ShapeKind::Square,
        },
    ]
}
