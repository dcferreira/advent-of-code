use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut inp_vec: Vec<i32> = vec![];

    for line in reader.lines() {
        inp_vec.push(line?.parse::<i32>().unwrap());
    }

    let windows = inp_vec.windows(3);
    let mut prev = 0;
    let mut cur: i32;
    let mut count = -1;
    for window in windows {
        cur = window.iter().sum();
        if cur > prev {
            count += 1;
        }
        prev = cur;
    }
    println!("count: {}", count);

    Ok(())
}
