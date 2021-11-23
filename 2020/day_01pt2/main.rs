use std::io;

type Record = (u32,);

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    let mut numbers: Vec<u32> = Vec::new();

    // add numbers to sets
    for result in reader.deserialize() {
        let record: Record = result?;
        numbers.push(record.0);
    }
    numbers.sort();

    for idx in 0..numbers.len() {
        let n = numbers[idx];
        for idx2 in idx..numbers.len() {
            let j = numbers[idx2];
            if n + 2 * j >= 2020 {
                continue;
            }
            for idx3 in idx2..numbers.len() {
                let k = numbers[idx3];
                if n + j + k > 2020 {
                    break;
                }
                if n + j + k == 2020 {
                    println!("result: {}", n * j * k);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
