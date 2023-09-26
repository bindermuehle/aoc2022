use std::collections::{HashSet, VecDeque};

use grid::Grid;

const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

type Coordinate = (usize, usize);

struct Map {
    map: Grid<Level>,
    start: Coordinate,
}
impl Map {
    fn new(lines: usize, cols: usize) -> Self {
        Self {
            map: Grid::new(lines, cols),
            start: (0, 0),
        }
    }
    fn parse(&mut self, input: &str) {
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, char)| {
                let level = Level::try_from(char).unwrap();
                match level.kind {
                    Kind::Start => self.start = (row, col),
                    _ => {}
                }
                self.map[row][col] = level;
            })
        });
    }
    fn find_shortest_path(&self, start: Coordinate) -> u64 {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut queue: VecDeque<Position> = VecDeque::new();
        let mut count = u64::MAX;
        queue.push_back(Position {
            coordinate: start,
            level: 0,
            steps: 0,
        });
        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            let row = pos.coordinate.0;
            let col = pos.coordinate.1;
            if row >= self.map.rows() || col >= self.map.cols() {
                continue;
            }
            let cell = &self.map[row][col];
            if visited.contains(&(row, col)) {
                continue;
            }

            if (cell.level as i16 - pos.level as i16) > 1 {
                continue;
            }
            match cell.kind {
                Kind::End => {
                    if pos.steps < count {
                        count = pos.steps;
                    }
                }
                _ => {
                    visited.insert((row, col));
                    DIRECTIONS.iter().for_each(|(r, c)| {
                        let r = row as isize + *r as isize;
                        let c = col as isize + *c as isize;
                        if r > 0 || c > 0 {
                            queue.push_back(Position {
                                coordinate: (r as usize, c as usize),
                                level: cell.level,
                                steps: pos.steps + 1,
                            });
                        }
                    })
                }
            }
        }
        return count;
    }
    fn find_most_scenic_path(&self) -> Option<u64> {
        self.map
            .indexed_iter()
            .filter(|((row, col), cell)| cell.level == 0)
            .map(|(cord, _)| self.find_shortest_path(cord))
            .min()
    }
}
struct Position {
    coordinate: Coordinate,
    level: u8,
    steps: u64,
}

#[derive(Debug, Default)]
enum Kind {
    Start,
    End,
    #[default]
    Default,
}
#[derive(Debug, Default)]
struct Level {
    level: u8,
    kind: Kind,
}

impl TryFrom<char> for Level {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' => Ok(Level {
                level: c as u8 - 'a' as u8,
                kind: Kind::Default,
            }),
            'S' => Ok(Level {
                level: 0,
                kind: Kind::Start,
            }),
            'E' => Ok(Level {
                level: 'z' as u8 - 'a' as u8,
                kind: Kind::End,
            }),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map = Map::new(
        input.lines().count(),
        input.lines().nth(1).unwrap().chars().count(),
    );

    map.parse(&input);
    let count = map.find_most_scenic_path();
    if let Some(count) = count {
        println!("Part 2: {}", count);
    }
}
