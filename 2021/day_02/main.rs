use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut horizontal = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let s = line?;
        let inp: Vec<&str> = s.split(" ").collect();
        let direction = inp[0];
        let distance = inp[1].parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => {}
        }
    }
    println!("result: {}", horizontal * depth);

    Ok(())
}
