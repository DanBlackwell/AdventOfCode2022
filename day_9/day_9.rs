use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut chain: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 { chain.push((0,0)); }
    
    let mut tail_covered_1: HashSet<(i32, i32)> = HashSet::new();
    tail_covered_1.insert((0, 0));
    
    let mut tail_covered_9: HashSet<(i32, i32)> = HashSet::new();
    tail_covered_9.insert((0, 0));
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let v: Vec<&str> = line.split(' ').collect();
                let direction = v[0];
                let distance = v[1];
                let mut to_move = distance.parse::<usize>().unwrap();
                
                let move_func = match direction {
                    "U" => |x: &mut (i32, i32)| { x.1 += 1; },
                    "D" => |x: &mut (i32, i32)| { x.1 -= 1; },
                    "L" => |x: &mut (i32, i32)| { x.0 -= 1; },
                    "R" => |x: &mut (i32, i32)| { x.0 += 1; },
                     _  => |_: &mut (i32, i32)| { panic!("Unexpected direction"); }
                };
                
                while to_move > 0 {
                    move_func(&mut chain[0]);
                    to_move -= 1;
                    
                    for i in 1..10 {
                        chain[i] = update_pos(chain[i-1], chain[i], direction);
                        // println!("direction: {direction}, to_move: {to_move}, chain[{}]: {:?}, chain[{i}]: {:?}", i-1, chain[i-1], chain[i]);
                    }
                    
                    if tail_covered_1.insert(chain[1]) {
                        // println!("Added {:?}", chain[1]);
                    }
                    
                    if tail_covered_9.insert(chain[9]) {
                        // println!("Added {:?}", chain[9]);
                    }
                }
            }
        }
    }
    
    println!("Total covered by 1: {}", tail_covered_1.len());
    println!("Total covered by 9: {}", tail_covered_9.len());
}

fn update_pos(head: (i32, i32), tail: (i32, i32), direction: &str) -> (i32, i32) {
    let (head_x, head_y) = head;
    let (mut tail_x, mut tail_y) = tail;
    
    if head_x == tail_x {
        if (head_y - tail_y).abs() > 1 {
            if head_y > tail_y {
                tail_y += 1;
            } else if head_y < tail_y {
                tail_y -= 1;
            } else {
                if direction == "U" { tail_y += 1; } else { tail_y -= 1; }
            }
        }
    } else if head_y == tail_y {
        if (head_x - tail_x).abs() > 1 {
            if head_x > tail_x {
                tail_x += 1;
            } else if head_x < tail_x {
                tail_x -= 1;
            } else {
                if direction == "R" { tail_x += 1; } else { tail_x -= 1; }
            }
        }
    } else /* diagonal */ {
        if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
            if head_x > tail_x { tail_x += 1; } else { tail_x -= 1; }
            if head_y > tail_y { tail_y += 1; } else { tail_y -= 1; }
        } 
    }
    
    return (tail_x, tail_y);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
