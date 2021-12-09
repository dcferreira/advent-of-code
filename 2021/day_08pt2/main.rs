#![warn(clippy::all)]
use std::collections::HashMap;

fn get_digit(s: &str, char_count_map: &HashMap<char, usize>) -> u8 {
    let sum = s.chars().map(|c| char_count_map[&c]).sum();
    match sum {
        42 => 0,
        17 => 1,
        34 => 2,
        39 => 3,
        30 => 4,
        37 => 5,
        41 => 6,
        25 => 7,
        49 => 8,
        45 => 9,
        _ => 127,
    }
}

fn get_digits_map(s: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::<char, usize>::new();
    for c in "abcdefg".chars() {
        let count = s.chars().filter(|x| *x == c).count();
        char_count.insert(c, count);
    }
    char_count
}

fn main() {
    let input = include_str!("./data/input.txt");
    let mut sum = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split('|').collect();
        let map = get_digits_map(split[0]);
        let mut digits: Vec<u8> = vec![];
        for digit_string in split[1].split_whitespace() {
            digits.push(get_digit(digit_string, &map));
        }
        let mut n = 0;
        let base: u32 = 10;
        for i in 0..4 {
            n += (digits[3 - i] as u32) * base.pow(i as u32);
        }
        sum += n;
    }
    println!("{}", sum);
}
