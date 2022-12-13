use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elevation: Vec<Vec<u8>> = Vec::new();
    let mut start_coord: (usize, usize) = (0, 0);
    let mut end_coord: (usize, usize) = (0, 0);

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let mut new_row = Vec::new();
                for letter in line.chars() {
                    match letter {
                        'S' => { start_coord = (new_row.len(), elevation.len()); new_row.push(1);  },
                        'E' => { end_coord =   (new_row.len(), elevation.len()); new_row.push(26); },
                         _  => new_row.push((letter as u8) - ('a' as u8) + 1),
                    }
                }
                elevation.push(new_row);
            }
        }
    }

    let mut distance = Vec::new();
    for _ in 0..elevation.len() { 
        let mut new_row = Vec::new();
        for _ in 0..elevation[0].len() { new_row.push(1_000_000); }
        distance.push(new_row);
    }
 
    let mut dist_part_2 = distance.clone();

    distance[start_coord.1][start_coord.0] = 0;
    dist_part_2[end_coord.1][end_coord.0] = 0;

    bfs(&elevation, &mut distance, 0, start_coord, end_coord, false); 
    println!("shortest route: {}", distance[end_coord.1][end_coord.0]);

    bfs(&elevation, &mut dist_part_2, 0, end_coord, start_coord, true); 
    let mut shortest_path = 1_000_000;
    for (row_idx, row) in dist_part_2.into_iter().enumerate() {
        for (col_idx, dist) in row.into_iter().enumerate() {
            if elevation[row_idx][col_idx] == 1 && dist < shortest_path {
                shortest_path = dist;
            }
        }
    }
    println!("Shortest path: {shortest_path}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn bfs(elevation: &Vec<Vec<u8>>, distance: &mut Vec<Vec<usize>>, depth: usize, cur_coord: (usize, usize), end_coord: (usize, usize), part_2: bool) {
    if cur_coord == end_coord { return; }
    let (x, y) = cur_coord;

    let cur_height = elevation[y][x];
    if part_2 && cur_height == 1 { return; }

    let mut options = Vec::new();
    if x > 0 { options.push((x - 1, y)); } 
    if x < elevation[0].len() - 1 { options.push((x + 1, y)); }
    if y > 0 { options.push((x, y - 1)); } 
    if y < elevation.len() - 1 { options.push((x, y + 1)); }
    
    for (next_x, next_y) in options {
        let next_elev = elevation[next_y][next_x];

        let mut is_valid = distance[next_y][next_x] > depth + 1;
        if !part_2 { 
           is_valid &= next_elev <= cur_height + 1;
        } else {
           is_valid &= cur_height <= next_elev + 1;
        }

        if is_valid {
            distance[next_y][next_x] = depth + 1;
            bfs(&elevation, distance, depth + 1, (next_x, next_y), end_coord, part_2);
        }
    }
}
