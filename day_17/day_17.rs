use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let mut rocks = Vec::new();

    rocks.push(Vec::from([(0,0),(1,0),(2,0),(3,0)]));
    rocks.push(Vec::from([(1,0),(0,1),(1,1),(2,1),(1,2)]));
    rocks.push(Vec::from([(0,0),(1,0),(2,0),(2,1),(2,2)]));
    rocks.push(Vec::from([(0,0),(0,1),(0,2),(0,3)]));
    rocks.push(Vec::from([(0,0),(1,0),(0,1),(1,1)]));

    let mut grid: [Vec<bool>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut moves: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                moves = line.chars().collect::<Vec<char>>();
            }
        }
    }

    fn print_grid(grid: &[Vec<bool>; 7], rock_coords: &Vec<(usize, usize)>) {
        for row in (0..grid[0].clone().len()).rev() {
            for col in 0..grid.len() {
                if rock_coords.iter().filter(|&r| *r == (col, row)).count() > 0 {
                    print!("@");
                } else {
                    print!("{}", if grid[col][row] { "#" } else { "." });
                }
            }
            println!("");
        }
        println!("");
    }


    let mut highest = 0;
    let mut rock_count = 0;
    let mut new_rock = true;
    let mut rock_coords = Vec::new();
    let mut move_index = 0;
    let mut stack_heights = Vec::new();
    loop {
        let mov = moves[move_index % moves.len()];
        move_index += 1;
        if new_rock {
            new_rock = false;

            if highest + 7 > grid[0].clone().len() {
                for col in &mut grid { 
                    for _ in 0..7 { col.push(false); }
                }
            }

            let rock_raw = rocks[rock_count % 5].clone();
            rock_coords = rock_raw
                .iter()
                .map(|(x,y)| (x + 2, y + highest + 3))
                .collect::<Vec<(usize, usize)>>();

            rock_count += 1;
        }

        let can_move_lr = rock_coords.iter().fold(true, |can_move, (x,y)| {
            if !can_move { return false; }

            return match mov {
                '<' => *x > 0 && !grid[*x - 1][*y],
                '>' => *x < 6 && !grid[*x + 1][*y],
                 _  => panic!()
            }
        });

        if can_move_lr {
            rock_coords = rock_coords.iter().map(|(x,y)| 
                match mov {
                    '<' => (*x - 1, *y),
                    '>' => (*x + 1, *y),
                     _  => panic!(),
                }
            ).collect::<Vec<(usize, usize)>>();
        }

        let can_move_down = rock_coords.iter()
            .fold(true, |can_move, (x,y)| can_move && *y > 0 && !grid[*x][*y - 1]);

        if can_move_down {
            rock_coords = rock_coords.iter()
                .map(|(x,y)| (*x, *y - 1))
                .collect::<Vec<(usize, usize)>>();
        }  else {
            let prev_highest = highest;

            rock_coords.iter().for_each(|(x,y)| {
                if *y + 1 > highest { highest = *y + 1; }
                grid[*x][*y] = true;
            });
            new_rock = true;

            stack_heights.push(highest - prev_highest);

            let stack_heights_len = stack_heights.clone().len();
            let mut periods_found = 0;
            const CHECK_DEPTH: usize = 50;
            if stack_heights_len > 1000 {
                let latest_heights = stack_heights[(stack_heights_len - CHECK_DEPTH - 1)..].to_vec();
                for start in 0..(stack_heights_len - 2 * (CHECK_DEPTH + 1)) {
                    let mut all_match = true;
                    for i in 0..CHECK_DEPTH {
                        if latest_heights[i] != stack_heights[start + i] {
                            all_match = false;
                            break;
                        }
                    }

                    if all_match { 
                        let period = stack_heights_len - start - CHECK_DEPTH - 1;
                        println!("period: {period}");

                        let height_gained = stack_heights[start..(stack_heights_len - CHECK_DEPTH - 1)].iter()
                            .fold(0, |total, step| total + step);
                        println!("height gained between {start} and {} = {height_gained}", stack_heights_len - CHECK_DEPTH - 1);

                        let offset = (1_000_000_000_000 - start) % period;
                        let height_to_offset = stack_heights[start..start + offset].iter()
                            .fold(0, |total, step| total + step);
                        let num_periods = (1_000_000_000_000 - start) / period;

                        let height_to_start = stack_heights[0..start].iter()
                            .fold(0, |total, step| total + step);
                        println!("Need to do another {} reps, which is another {} from now", 1_000_000_000_000 - rock_count, offset);
                        let total_height = height_to_start + height_gained * num_periods + height_to_offset;

                        println!("total_height: {total_height}");
                        return;
                    }
                }
            }

            if rock_count % 5 == 0 {
                let mut all_flat = true;
                for i in 0..grid.len() {
                    all_flat &= grid[i][highest - 1];
                }
                if all_flat {
                    print_grid(&grid, &rock_coords);
                    panic!();
                }
            }
            if rock_count == 2022 { println!("part 1: {}", highest);}
        }
    }

    println!("highest: {highest}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
