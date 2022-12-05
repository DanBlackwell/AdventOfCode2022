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
            const WIN: char = 'Z'; const DRAW: char = 'Y'; const LOSE: char = 'X';
            let rock_pts = 1; let paper_pts = 2; let scissor_pts = 3;

            let score = match opp {
                /*rock*/    'A' => match us { WIN => 6 + paper_pts,   DRAW => 3 + rock_pts,    LOSE => 0 + scissor_pts, _ => -1 },
                /*paper*/   'B' => match us { WIN => 6 + scissor_pts, DRAW => 3 + paper_pts,   LOSE => 0 + rock_pts,    _ => -1 },
                /*scissor*/ 'C' => match us { WIN => 6 + rock_pts,    DRAW => 3 + scissor_pts, LOSE => 0 + paper_pts,   _ => -1 },
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
