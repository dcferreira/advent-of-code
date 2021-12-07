fn main() {
    let input = include_str!("./data/input.txt").trim();
    let mut initial_crabs: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    initial_crabs.sort();

    let h_position = initial_crabs[initial_crabs.len() / 2];
    let fuel: i32 = initial_crabs.iter().map(|x| (x - h_position).abs()).sum();
    println!("fuel: {}", fuel);
}
