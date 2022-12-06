use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const CHALLENGE_PART_2: bool = true;
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let mut uniques = String::from("");
                for (index, letter) in line.chars().enumerate() {
                    if let Some(pos) = uniques.find(letter) {
                        let mut remaining: String;
                        { 
                            let (_, rem) = uniques.split_at(pos + 1);
                            remaining = rem.to_string();
                            remaining += format!("{letter}").as_str();
                        }
                        uniques = remaining;
                        // println!("Found duplicate, reduced uniques to {uniques}");
                    } else {
                        uniques.push(letter);
                        // println!("increased uniques to {uniques}");
                        
                        if uniques.len() == if CHALLENGE_PART_2 { 14 } else { 4 } { 
                            println!("Found 4 letters unique: {uniques}, index: {}", index + 1);
                            break;
                        }
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
