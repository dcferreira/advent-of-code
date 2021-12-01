use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut prev = 0;
    let mut cur: i32;
    let mut count = -1;

    for line in reader.lines() {
        cur = line?.parse::<i32>().unwrap();
        if cur > prev {
            count += 1;
        }
        prev = cur;
    }
    println!("count: {}", count);

    Ok(())
}
