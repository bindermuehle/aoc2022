use std::collections::HashSet;

use grid::Grid;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut grid: Grid<u8> = Grid::new(
        input.lines().count(),
        input.lines().nth(1).unwrap().chars().count(),
    );
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let num = (c.to_string()).parse::<u8>().unwrap();
            grid[i][j] = num;
        });
    });

    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for row in 1..grid.rows() - 1 {
        get_visible(grid.iter_row(row).collect())
            .into_iter()
            .for_each(|i| {
                visible.insert((row, i));
            });
    }

    for col in 1..grid.cols() - 1 {
        get_visible(grid.iter_col(col).collect())
            .into_iter()
            .for_each(|i| {
                visible.insert((i, col));
            })
    }

    let size = (grid.rows() + grid.cols() - 2) * 2 + visible.len();
    println!("count : {:?}", size);
}
fn get_visible(list: Vec<&u8>) -> Vec<usize> {
    let mut visible = vec![];
    let mut first = list[0];
    let mut last = list[list.len() - 1];
    for i in 1..list.len() - 1 {
        if first < list[i] {
            first = list[i];
            visible.push(i);
        }
    }
    for i in (1..list.len() - 1).rev() {
        if last < list[i] {
            last = list[i];
            visible.push(i);
        }
    }
    return visible;
}
