#![warn(clippy::all)]
use grid::{grid, Grid};
use std::collections::HashSet;

fn main() {
    let input = include_str!("./data/input.txt");
    let mut depths: Grid<u8> = grid![];
    for line in input.lines() {
        depths.push_row(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    let mut horizontal_mins: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..depths.rows() {
        let row = depths.iter_row(i).collect::<Vec<&u8>>();
        if row[0] < row[1] {
            horizontal_mins.insert((i, 0));
        }
        for (j, ns) in row[..].windows(3).enumerate() {
            if (ns[0] > ns[1]) & (ns[1] < ns[2]) {
                horizontal_mins.insert((i, j + 1));
            }
        }
        if row[(row.len() - 1) as usize] < row[(row.len() - 2) as usize] {
            horizontal_mins.insert((i, row.len() - 1));
        }
    }

    let mut vertical_mins: HashSet<(usize, usize)> = HashSet::new();
    for j in 0..depths.cols() {
        let col = depths.iter_col(j).collect::<Vec<&u8>>();
        if col[0] < col[1] {
            vertical_mins.insert((0, j));
        }
        for (i, ns) in col[..].windows(3).enumerate() {
            if (ns[0] > ns[1]) & (ns[1] < ns[2]) {
                vertical_mins.insert((i + 1, j));
            }
        }
        if col[(col.len() - 1) as usize] < col[(col.len() - 2) as usize] {
            vertical_mins.insert((col.len() - 1, j));
        }
    }
    let mins = horizontal_mins.intersection(&vertical_mins);
    let mut basins: Vec<u32> = mins
        .map(|m| {
            get_basin_size(
                *m,
                &depths,
                &mut Grid::from_vec(vec![false; depths.rows() * depths.cols()], depths.cols()),
            )
        })
        .collect();
    basins.sort_unstable();
    basins.reverse();
    println!("{:?}", basins);
    println!("{:?}", basins[..3].iter().product::<u32>());
}

fn get_basin_size(pos: (usize, usize), depths: &Grid<u8>, visited: &mut Grid<bool>) -> u32 {
    if visited[pos.0][pos.1] {
        return 0;
    }
    visited[pos.0][pos.1] = true;
    let rows = depths.rows() - 1;
    let cols = depths.cols() - 1;
    match pos {
        _ if depths[pos.0][pos.1] == 9 => 0,
        (0, 0) => {
            1 + get_basin_size((pos.0, pos.1 + 1), depths, visited)
                + get_basin_size((pos.0 + 1, pos.1), depths, visited)
        }
        (0, c) if c == cols => {
            1 + get_basin_size((pos.0 + 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 - 1), depths, visited)
        }
        (r, 0) if r == rows => {
            1 + get_basin_size((pos.0 - 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 + 1), depths, visited)
        }
        (r, c) if (r == rows) & (c == cols) => {
            1 + get_basin_size((pos.0 - 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 - 1), depths, visited)
        }
        (0, _) => {
            1 + get_basin_size((pos.0 + 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 + 1), depths, visited)
                + get_basin_size((pos.0, pos.1 - 1), depths, visited)
        }
        (_, 0) => {
            1 + get_basin_size((pos.0 + 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 + 1), depths, visited)
                + get_basin_size((pos.0 - 1, pos.1), depths, visited)
        }
        (r, _) if r == rows => {
            1 + get_basin_size((pos.0, pos.1 + 1), depths, visited)
                + get_basin_size((pos.0 - 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 - 1), depths, visited)
        }
        (_, c) if c == cols => {
            1 + get_basin_size((pos.0 + 1, pos.1), depths, visited)
                + get_basin_size((pos.0 - 1, pos.1), depths, visited)
                + get_basin_size((pos.0, pos.1 - 1), depths, visited)
        }
        _ => {
            1 + get_basin_size((pos.0, pos.1 + 1), depths, visited)
                + get_basin_size((pos.0, pos.1 - 1), depths, visited)
                + get_basin_size((pos.0 + 1, pos.1), depths, visited)
                + get_basin_size((pos.0 - 1, pos.1), depths, visited)
        }
    }
}
