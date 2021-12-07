fn main() {
    let input = include_str!("./data/input.txt").trim();
    let initial_crabs: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let avg = initial_crabs.iter().sum::<i32>() as f32 / initial_crabs.len() as f32;
    let h_position = avg.floor() as i32;
    let fuel_array: Vec<i32> = initial_crabs
        .iter()
        .map(|x| (1..=((x - h_position).abs())).sum::<i32>())
        .collect();
    println!("final position: {}", h_position);
    println!("fuel: {}", fuel_array.iter().sum::<i32>());
}
