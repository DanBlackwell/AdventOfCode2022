use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(PartialEq)]
enum SquareContents {
    Empty,
    Wall,
    Sand,
}

fn main() {
    let mut path_coords: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut grid: Vec<Vec<SquareContents>> = Vec::new();
    let mut min_x = 1_000_000;
    let mut max_x = 0;
    let mut min_y = 1_000_000;
    let mut max_y = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let mut new_path = Vec::new();
                let coords = line.split(" -> ");
                for coord in coords {
                    let parts = coord.split(",").collect::<Vec<&str>>();
                    
                    let x = parts[0].parse::<usize>().unwrap();
                    if x < min_x { min_x = x; }
                    if x > max_x { max_x = x; }
                    
                    let y = parts[1].parse::<usize>().unwrap();
                    if y < min_y { min_y = y; }
                    if y > max_y { max_y = y; }
                    
                    new_path.push((x, y));
                }
                path_coords.push(new_path);
            }
        }
    }
    
    // println!("min_x: {min_x}, max_x: {max_x}, min_y: {min_y}, max_y: {max_y}");
    for row in 0..=max_y+2 {
        let mut grid_row = Vec::new();
        for col in 0..=(2 * max_x) {
            grid_row.push(if row == max_y+2 { SquareContents::Wall} else { SquareContents::Empty });
        }
        grid.push(grid_row);
    }
    
    for mut path in path_coords {
        // println!("new path");
        let mut prev_coord = path.pop().unwrap();
        for coord in path.into_iter().rev() {
            // println!("Drawing from {:?} to {:?}", prev_coord, coord);
            if prev_coord.0 != coord.0 {
                let start = std::cmp::min(prev_coord.0, coord.0);
                let end   = std::cmp::max(prev_coord.0, coord.0);
                for i in start..=end {
                    grid[coord.1][i] = SquareContents::Wall;
                }
            } else if prev_coord.1 != coord.1 {
                let start = std::cmp::min(prev_coord.1, coord.1);
                let end   = std::cmp::max(prev_coord.1, coord.1);
                for i in start..=end {
                    grid[i][coord.0] = SquareContents::Wall;
                }
            } else {
                panic!();
            }
            prev_coord = coord;
        }
    }
    
    print_grid(&grid);
    
    let mut sand_count = 0;
    let mut enter_the_void = false;
    let mut plugged_the_entrance = false;
    while !plugged_the_entrance {
        let mut sand_pos = (500,0);
        
        while true {
            let (x, y) = sand_pos;
            if !enter_the_void && y >= max_y {
                println!("reached the void after {sand_count} grains");
                enter_the_void = true;
            }
            
            if y == max_y +1 {
                sand_count += 1;
                grid[y][x] = SquareContents::Sand;
                break;
            }
            
            if grid[y+1][x] == SquareContents::Empty {
                sand_pos = (x, y+1);
            } else if grid[y+1][x-1] == SquareContents::Empty {
                sand_pos = (x-1, y+1);
            } else if grid[y+1][x+1] == SquareContents::Empty {
                sand_pos = (x+1, y+1);
            } else {
                sand_count += 1;
                grid[y][x] = SquareContents::Sand;
                if y == 0 { plugged_the_entrance = true; }
                break;
            }
        }
    }
    
    println!("Filled the entrance after {sand_count} grains");
    print_grid(&grid);
}

fn print_grid(grid: &Vec<Vec<SquareContents>>) {
    for row in grid {
        for col in row {
            print!("{}", match col {
                SquareContents::Empty => ".",
                SquareContents::Wall  => "#",
                SquareContents::Sand  => "o",
            });
        }
        print!("\n");
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
