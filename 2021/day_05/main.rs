use grid::Grid;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(s: &str) -> Self {
        let split: Vec<&str> = s.split(",").collect();
        return Point {
            x: split[0].parse::<usize>().unwrap(),
            y: split[1].parse::<usize>().unwrap(),
        };
    }
}

fn parse_coords(s: String) -> (Point, Point) {
    let s_split = s.split_once(" -> ").unwrap();
    let points = (Point::new(s_split.0), Point::new(s_split.1));
    return points;
}

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut input_points: Vec<(Point, Point)> = vec![];
    let mut y_size = 0;
    let mut x_size = 0;
    for l in reader.lines() {
        let line = l?;
        let point_pair = parse_coords(line);
        if !(point_pair.0.x == point_pair.1.x || point_pair.0.y == point_pair.1.y) {
            // skip if non horizontal/vertical
            continue;
        }
        // update grid size
        let max_x = max(point_pair.0.x, point_pair.1.x);
        let max_y = max(point_pair.0.y, point_pair.1.y);
        if max_x > x_size {
            x_size = max_x;
        }
        if max_y > y_size {
            y_size = max_y;
        }
        // push points
        input_points.push(point_pair);
    }

    let max_coord = max(x_size, y_size) + 1 as usize;
    let mut grid = Grid::from_vec(vec![0 as u32; max_coord * max_coord], max_coord);
    for point_pair in input_points.iter() {
        let start = &point_pair.0;
        let end = &point_pair.1;

        match (start.x == end.x, start.y == end.y) {
            (true, false) => {
                // println!("V {:?}, {:?}", start, end);
                let x = start.x;
                let from = min(start.y, end.y);
                let to = max(start.y, end.y) + 1;
                for y in from..to {
                    grid[x][y] += 1;
                }
            }
            (false, true) => {
                // println!("H {:?}, {:?}", start, end);
                let y = start.y;
                let from = min(start.x, end.x);
                let to = max(start.x, end.x) + 1;
                for x in from..to {
                    grid[x][y] += 1;
                }
            }
            _ => {
                println!("CANT GO HERE")
            }
        }
    }

    println!("count: {}", grid.iter().filter(|x| x > &&1).count());

    Ok(())
}
