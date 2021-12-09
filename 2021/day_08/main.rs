use std::collections::HashSet;

struct Number {
    pieces: HashSet<char>,
}

impl Number {
    pub fn new(s: &str) -> Self {
        let mut charset: HashSet<char> = HashSet::new();
        for c in s.chars() {
            charset.insert(c);
        }

        return Number {
            pieces: charset.clone(),
        };
    }

    fn get_nr(&self) -> Option<u8> {
        match &self.pieces {
            set if set.len() == 2 => Some(1),
            set if set.len() == 4 => Some(4),
            set if set.len() == 3 => Some(7),
            set if set.len() == 7 => Some(8),
            _ => None,
        }
    }
}

fn main() {
    let input = include_str!("./data/input.txt");
    let mut count = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split("|").collect();
        for digit in split[1].split_whitespace() {
            let nr = Number::new(digit);
            if let Some(_) = nr.get_nr() {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
