use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut counts0: Vec<u32> = vec![];
    let mut counts1: Vec<u32> = vec![];

    let mut flag = false;
    for line in reader.lines() {
        let s = line?;
        // initialize counts
        if !flag {
            for _bit in s.chars() {
                counts0.push(0);
                counts1.push(0);
            }
            flag = true;
        }

        // fill in counts
        for iter in s.chars().enumerate() {
            let bit = iter.1;
            let i = iter.0;
            let n = bit.to_digit(10).unwrap();
            if n == 0 {
                counts0[i] += 1;
            } else {
                counts1[i] += 1;
            }
        }
    }

    let base: i32 = 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for iter in counts0.iter().rev().zip(counts1.iter().rev()).enumerate() {
        let n = iter.0 as u32;
        let c = iter.1;
        let c0 = c.0;
        let c1 = c.1;

        if c0 > c1 {
            epsilon += base.pow(n);
            println!("0: {}, ({}, {})", base.pow(n), epsilon, gamma);
        } else {
            gamma += base.pow(n);
            println!("1: {}, ({}, {})", base.pow(n), epsilon, gamma);
        }
    }
    println!("eps: {}, gamma: {}", epsilon, gamma);
    println!("result: {}", epsilon * gamma);

    Ok(())
}
