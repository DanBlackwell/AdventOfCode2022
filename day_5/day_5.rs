use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    const CHALLENGE_PART_2: bool = true;
    let mut loaded_init = false;
    let mut stacks_count = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if !loaded_init {
                    if stacks_count == 0 { 
                        stacks_count = (line.len() + 1) / 4; 
                        for _ in 0..stacks_count {
                            stacks.push(Vec::new());
                        }
                    }
                    
                    for n in 0..stacks_count {
                        let letter = line.chars().nth(1 + 4 * n).unwrap();
                        if letter == '1' { 
                            loaded_init = true; 
                            for i in 0..stacks_count { stacks[i].reverse(); println!("stack {i}: {:?}", stacks[i]); }
                            break; 
                        }
                        else if letter != ' ' { stacks[n].push(letter); }
                    }
                } else if line.len() == 0 {
                } else {
                    let words: Vec<&str> = line.split(" ").collect();
                    let move_size = words[1].parse::<usize>().unwrap();
                    let source = words[3].parse::<usize>().unwrap() - 1;
                    let dest = words[5].parse::<usize>().unwrap() - 1;
                    
                    if CHALLENGE_PART_2 {
                        let mut move_vec = Vec::new();
                        {
                        let len = stacks[source].len();
                        let move_drain = &stacks[source].drain(len - move_size..);
                        move_vec.extend_from_slice(move_drain.as_slice());
                        }
                        stacks[dest].extend_from_slice(&move_vec[0..]);
                    } else {
                        for _ in 0..move_size {
                            let popped = stacks[source].pop().unwrap();
                            stacks[dest].push(popped);
                        }
                    }
                }
            }
        }
    }
    
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
