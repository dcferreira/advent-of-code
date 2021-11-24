use std::io;

type Record = (String,);

fn check_policy(minimum: usize, maximum: usize, c: char, s: &str) -> bool {
    let cond1 = s.chars().nth(minimum).unwrap() == c;
    let cond2 = s.chars().nth(maximum).unwrap() == c;
    return cond1 ^ cond2;
}

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    let mut count: u16 = 0;
    for result in reader.deserialize() {
        let record: Record = result?;
        let strings: Vec<&str> = record.0.split(':').collect();
        let policy_strs: Vec<&str> = (strings[0]).split(&[' ', '-'][..]).collect();

        if check_policy(
            policy_strs[0].parse::<usize>().unwrap(),
            policy_strs[1].parse::<usize>().unwrap(),
            policy_strs[2].chars().next().unwrap(),
            strings[1],
        ) {
            count += 1;
        }
    }

    println!("result: {}", count);
    Ok(())
}
