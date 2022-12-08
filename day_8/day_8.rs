use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    let mut prev_row = Vec::new();
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let mut new_row = Vec::new();
                let mut next_row = Vec::new();
                let mut vis = Vec::new();
                let mut left_max = 0;
                for (column, letter) in line.chars().enumerate() {
                    let num = letter.to_string().parse::<usize>().unwrap();
                    
                    let mut is_visible = column == 0 || num > left_max;
                    if num > left_max { left_max = num; }
                    
                    if grid.len() > 0 {
                        let above_max = prev_row[column]; 
                        if num > above_max { is_visible = true; next_row.push(num); } else { next_row.push(above_max); }
                    } else {
                        next_row.push(num);
                        is_visible = true;
                    }
                    
                    vis.push(is_visible);
                    new_row.push(num);
                }
                
                prev_row = next_row;
                grid.push(new_row);
                visible.push(vis);
            }
        }
    }
    
    for row_index in (0..grid.len()).rev() {
        let mut right_max = 0;
        let mut next_row = Vec::with_capacity(grid[0].len());
        for _ in 0..grid[0].len() { next_row.push(0); }
        
        for col_index in (0..grid[row_index].len()).rev() {
            let num = grid[row_index][col_index];
            let mut is_visible = col_index == grid[row_index].len() - 1 || num > right_max;
            
            if num > right_max { right_max = num; }
            
            if row_index < grid.len() - 1 {
                let below_max = prev_row[col_index];
                if num > below_max { 
                    is_visible = true; 
                    next_row[col_index] = num; 
                } else { 
                    next_row[col_index] = below_max; 
                }
            } else {
                next_row[col_index] = num; 
                is_visible = true;
            }
            
            visible[row_index][col_index] |= is_visible;
        }
        
        prev_row = next_row;
    }
    
    let mut part2score = 0;
    for row_index in 1..(grid.len() - 1) {
        for col_index in 1..(grid[0].len() - 1) { 
            let mut score = 1;
            let height = grid[row_index][col_index];
            
            // up
            for i in (0..row_index).rev() {
                if grid[i][col_index] >= height || i == 0 {
                    score *= row_index - i;
                    break;
                }
            }
            
            // down
            for i in (row_index + 1)..grid.len() {
                if grid[i][col_index] >= height || i == grid.len() - 1 {
                    score *= i - row_index;
                    break;
                }
            }
            
            // left 
            for i in (0..col_index).rev() {
                if grid[row_index][i] >= height || i == 0 {
                    score *= col_index - i;
                    break;
                }
            }
            
            // right 
            for i in (col_index + 1)..grid[0].len() {
                if grid[row_index][i] >= height || i == grid[0].len() - 1 {
                    score *= i - col_index;
                    break;
                }
            }
            
            if score > part2score {
                println!("Found new best candidate ({col_index}, {row_index}), score: {score}, height: {height}");
                part2score = score;
            }
        }
    }
    
//     println!("GRID:");
//     for row in grid {
//         for column in row {
//             print!("{column}");
//         }
//         println!("");
//     }
//     
//     
//     println!("LEFT:");
//     for row in highestFromLeft {
//         for column in row {
//             print!("{column}");
//         }
//         println!("");
//     }
//     
//     println!("TOP:");
//     for row in highestFromTop {
//         for column in row {
//             print!("{column}");
//         }
//         println!("");
//     }
//     
//     println!("BOTTOM:");
//     for row in highestFromBottom {
//         for column in row {
//             print!("{column}");
//         }
//         println!("");
//     }
//     
//     println!("RIGHT:");
//     for row in highestFromRight {
//         for column in row {
//             print!("{column}");
//         }
//         println!("");
//     }
    
    let mut num_visible = 0;
    // println!("VISIBLE:");
    for row in visible {
        for column in row {
            // print!("{}", if column { "T" } else { "F" });
            if column { num_visible += 1; }
        }
        // println!("");
    }
    
    println!("\nnum_visible: {num_visible}");
    println!("Part 2 best: {part2score}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
