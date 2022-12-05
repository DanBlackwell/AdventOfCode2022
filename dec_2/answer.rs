use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("inputs.txt")?;
    let reader = BufReader::new(file);

    let mut total_score: i32 = 0;
    for line in reader.lines() {
        let line = match line {
            Err(e) => return Err(e),
            Ok(l) => l,
        };

        if line.len() == 0 {
            println!("Found an empty line!");
        } else {
            let opp = line.chars().nth(0).unwrap();
            let us = line.chars().nth(2).unwrap();

            let score = match us {
                'X' => 1 + match opp { 'A' => 3, 'B' => 0, 'C' => 6, _ => -2 },
                'Y' => 2 + match opp { 'A' => 6, 'B' => 3, 'C' => 0, _ => -3 },
                'Z' => 3 + match opp { 'A' => 0, 'B' => 6, 'C' => 3, _ => -4 },
                _ => -1,
            };

            if score < 0 {
                panic!("Error occurred reading moves!");
            }

            total_score += score;
            println!("opp: {}, us: {}, score: {}", opp, us, score);
        }
    }

    println!("score: {}", total_score);
    Ok(())
}
