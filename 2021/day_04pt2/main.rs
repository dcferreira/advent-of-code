use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct BingoBoard {
    numbers: Vec<Vec<u32>>,
    marked_positions: Vec<Vec<bool>>,
}

impl BingoBoard {
    pub fn new() -> Self {
        BingoBoard {
            numbers: vec![],
            marked_positions: vec![],
        }
    }

    fn append_row(&mut self, row: &str) {
        let numbers_str = row.split_whitespace();
        let numbers: Vec<u32> = numbers_str.map(|x| x.parse::<u32>().unwrap()).collect();
        self.marked_positions.push(vec![false; numbers.len()]);
        self.numbers.push(numbers);
    }

    fn insert(&mut self, number: u32) -> bool {
        for (row_idx, row) in self.numbers.iter().enumerate() {
            let col = row.iter().position(|x| *x == number);
            if let None = col {
                continue;
            }
            self.marked_positions[row_idx][col.unwrap()] = true;
            // check if it's a full row
            if self.marked_positions[row_idx].iter().all(|x| *x) {
                return true;
            }
            //check if it's a full column
            if self
                .marked_positions
                .iter()
                .map(|x| x[col.unwrap()])
                .all(|x| x)
            {
                return true;
            }
        }
        return false;
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for (n_row, mark_row) in self.numbers.iter().zip(self.marked_positions.iter()) {
            for (n_col, mark_col) in n_row.iter().zip(mark_row.iter()) {
                if !*mark_col {
                    sum += *n_col
                }
            }
        }
        return sum;
    }
}

type FillInOutput = u32;
trait BingoVector {
    fn fill_in(&mut self, given_numbers: Vec<u32>, n_boards: usize) -> FillInOutput;
}

impl BingoVector for Vec<BingoBoard> {
    fn fill_in(&mut self, given_numbers: Vec<u32>, n_boards: usize) -> FillInOutput {
        let mut boards_won_set: HashSet<usize> = HashSet::new();
        let mut boards_won: Vec<usize> = vec![]; // keeps board indices
        let mut winning_numbers: Vec<u32> = vec![]; // keeps numbers
        for (_i, n) in given_numbers.iter().enumerate() {
            for (i, board) in self.iter_mut().enumerate() {
                if boards_won_set.contains(&i) {
                    continue;
                }
                if board.insert(*n) {
                    boards_won_set.insert(i);
                    boards_won.push(i);
                    winning_numbers.push(*n);
                    println!("{}: {}/{}", i, boards_won.len(), n_boards);
                }
            }
        }

        let winning_board_idx = *boards_won.last().unwrap();
        let sum = self[winning_board_idx].sum_unmarked();
        let n = winning_numbers.last().unwrap();
        println!("winning number: {}, sum: {}", n, sum);
        return sum * n;
    }
}

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let mut boards: Vec<BingoBoard> = vec![];

    let mut lines_iterator = reader.lines();
    let given_numbers: Vec<u32> = lines_iterator
        .next()
        .unwrap()?
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    lines_iterator.next(); // eat one empty line

    // create boards
    let mut board = BingoBoard::new();
    while let Some(l) = lines_iterator.next() {
        let line = l?;
        if line == "" {
            boards.push(board);
            board = BingoBoard::new();
            continue;
        }
        board.append_row(&line);
    }
    boards.push(board); // push the last one

    println!("{}", boards.len());
    println!("{}", boards.fill_in(given_numbers, boards.len()));

    Ok(())
}
