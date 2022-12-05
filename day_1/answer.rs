use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let file = File::open("inputs.txt")?;
    let reader = BufReader::new(file);
    let mut calories = Vec::new();
    let mut highest: u32 = 0;

    let mut total: u32 = 0;
    for line in reader.lines() {
        let line = match line {
            Err(e) => return Err(e),
            Ok(l) => l,
        };

        if line.len() == 0 {
            calories.push(total);
            if total > highest {
                highest = total;
            }
            total = 0;
        } else {
            let cals: u32 = line.parse().unwrap();
            total += cals;
        }
    }

    calories.sort();
    let top3 = &calories[calories.len() - 3..];
    println!("calories: {:?},\nhighest: {},\ntop 3: {:?} (sum: {})", calories, highest, top3, top3.iter().sum::<u32>());
    Ok(())
}
