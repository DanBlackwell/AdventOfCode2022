use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let mut grid = Vec::new();
    let mut grid_void_checked = Vec::new();
    for _ in 0..30 {
        let mut new_row = Vec::new();
        for _ in 0..30 {
            let new_col = Vec::from([false; 30]);
            new_row.push(new_col);
        }
        let new_row_clone = new_row.clone();
        grid.push(new_row);
        grid_void_checked.push(new_row_clone);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let cube_vec: Vec<usize> = line
                                .split(",")
                                .map(|a| a.parse::<usize>().unwrap())
                                .collect();

                grid[cube_vec[0] + 1][cube_vec[1] + 1][cube_vec[2] + 1] = true;
            }
        }
    }

    let mut faces = 0;
    for x in 0..30 {
        for y in 0..30 {
            for z in 0..30 {
                if !grid[x][y][z] { continue; }
                faces += 6;
                if x > 0 &&  grid[x-1][y][z] { faces -= 1; }
                if x < 29 && grid[x+1][y][z] { faces -= 1; }
                if y > 0 &&  grid[x][y-1][z] { faces -= 1; }
                if y < 29 && grid[x][y+1][z] { faces -= 1; }
                if z > 0 &&  grid[x][y][z-1] { faces -= 1; }
                if z < 29 && grid[x][y][z+1] { faces -= 1; }
            }
        }
    }

    let mut queue = Vec::from([(0,0,0)]);
    let mut edge_length = 0;

    while queue.len() > 0 {
        let (x,y,z) = queue.pop().unwrap();
        if grid[x][y][z] { edge_length += 1; continue; }
        if grid_void_checked[x][y][z] { continue; }
        grid_void_checked[x][y][z] = true;

        if x != 0  { queue.push((x-1,y,z)); }
        if x != 29 { queue.push((x+1,y,z)); }
        if y != 0  { queue.push((x,y-1,z)); }
        if y != 29 { queue.push((x,y+1,z)); }
        if z != 0  { queue.push((x,y,z-1)); }
        if z != 29 { queue.push((x,y,z+1)); }
    }

    println!("faces: {faces}");
    println!("edges: {edge_length}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
