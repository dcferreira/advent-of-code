use std::collections::HashSet;
use std::io;

type Record = (u32,);

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    let mut low: HashSet<u32> = HashSet::new(); // n smaller than 2020/2
    let mut high: HashSet<u32> = HashSet::new(); // n larger than 2020/2

    // add numbers to sets
    for result in reader.deserialize() {
        let record: Record = result?;
        let n = record.0;
        if n < 1010 {
            low.insert(n);
        } else {
            high.insert(n);
        }
    }

    println!("low: {:?}", low.len());
    println!("high: {:?}", high.len());

    // for numbers in the small set, check if anything adds to 2020
    for n in &low {
        if high.contains(&(2020 - n)) {
            println!("result: {}", n * (2020 - n))
        }
    }

    Ok(())
}
