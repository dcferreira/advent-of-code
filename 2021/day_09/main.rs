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
    // println!("{:?}", mins);
    let sum: u32 = mins.map(|t| depths[t.0][t.1] as u32 + 1).sum();
    println!("{}", sum);
}
