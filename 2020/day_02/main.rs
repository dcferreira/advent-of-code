use std::io;

type Record = (String,);

fn check_policy(minimum: u16, maximum: u16, c: &str, s: &str) -> bool {
    let count: u16 = s.matches(c).count().try_into().unwrap();
    if minimum <= count && count <= maximum {
        return true;
    }
    return false;
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
        println!("{:?}", policy_strs);

        if check_policy(
            policy_strs[0].parse::<u16>().unwrap(),
            policy_strs[1].parse::<u16>().unwrap(),
            policy_strs[2],
            strings[1],
        ) {
            count += 1;
        }
    }

    println!("result: {}", count);
    Ok(())
}
