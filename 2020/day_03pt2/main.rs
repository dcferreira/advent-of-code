use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    x_step: u16,
    #[structopt(default_value = "1")]
    y_step: u16,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_trees_in_line(line: String) -> HashSet<u16> {
    let mut indices: HashSet<u16> = HashSet::new();
    for m in line.match_indices("#") {
        indices.insert(m.0 as u16);
    }
    return indices;
}

fn main() {
    let args = Cli::from_args();

    // build map
    let mut length: u16 = 0;
    let mut map: Vec<HashSet<u16>> = vec![];
    if let Ok(lines) = read_lines("./data/input.txt") {
        for line_res in lines {
            if let Ok(line) = line_res {
                if length == 0 {
                    length = line.chars().count() as u16;
                }

                let res = get_trees_in_line(line);
                map.push(res);
            }
        }
    }

    // slide!
    let mut trees: u16 = 0;
    let mut x: u16 = 0;
    for y in (0..map.len()).step_by(args.y_step as usize) {
        if map[y].contains(&x) {
            trees += 1;
        }
        x = (x + args.x_step) % length;
    }

    println!("result: {}", trees);
}
