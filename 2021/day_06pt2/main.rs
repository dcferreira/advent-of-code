use std::collections::VecDeque;

fn main() {
    let input = include_str!("./data/input.txt");
    let initial_fish: Vec<u8> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut fish = VecDeque::with_capacity(7);
    for _f in 0..7 {
        fish.push_front(0);
    }
    for f in initial_fish.iter() {
        fish[*f as usize] += 1;
    }
    let mut fish_at_7: u64 = 0;
    let mut fish_at_8: u64 = 0;
    let mut new_fish_next_round;
    for _epoch in 0..256 {
        new_fish_next_round = fish[0];
        let fish_at_6 = fish_at_7 + fish[0];
        fish_at_7 = fish_at_8;
        fish_at_8 = new_fish_next_round;
        fish.pop_front();
        fish.push_back(fish_at_6);
        // println!(
        //     "epoch {}: ({}),{:?}",
        //     _epoch,
        //     fish.iter().sum::<u64>() + fish_at_7 + fish_at_8,
        //     fish
        // )
    }
    println!(
        "n_fish: {}",
        fish.iter().sum::<u64>() + fish_at_7 + fish_at_8
    );
}
