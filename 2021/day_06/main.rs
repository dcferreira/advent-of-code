type Age = u8;

fn main() {
    let input = include_str!("./data/input.txt");
    let mut fish: Vec<Age> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<Age>().unwrap())
        .collect::<Vec<_>>();

    let mut new_fish;
    for _epoch in 0..80 {
        new_fish = fish.iter().filter(|x| **x == 0).count();
        fish = fish
            .iter()
            .map(|x| if *x == 8 { 7 } else { (*x + 7 - 1) % 7 })
            .collect();

        let mut new_fish_vec: Vec<Age> = vec![8; new_fish];
        fish.append(&mut new_fish_vec);
        // println!("epoch {}: ({}),{:?}", epoch, fish.len(), fish);
    }
    println!("n_fish: {}", fish.len());
}
