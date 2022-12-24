use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Blizzard {
    coord: Coord,
    direction: Direction
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum CellContents {
    Wall,
    Blizzards(Vec<Blizzard>)
}

fn main() {
    let mut grid = Vec::new();
    let mut loaded = false;
    let mut start_pos = Coord { x: 0, y: 0 };
    let mut end_pos = Coord { x: 0, y: 0 };

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.into_iter().enumerate() {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }

                let mut new_row = Vec::new();
                
                for (column, letter) in line.chars().enumerate() {
                    let coord = Coord { x: column, y: line_num };
                    match letter {
                        '#' => new_row.push(CellContents::Wall),
                        '^' => new_row.push(CellContents::Blizzards(Vec::from([Blizzard { coord: coord, direction: Direction::Up }]))),
                        'v' => new_row.push(CellContents::Blizzards(Vec::from([Blizzard { coord: coord, direction: Direction::Down }]))),
                        '<' => new_row.push(CellContents::Blizzards(Vec::from([Blizzard { coord: coord, direction: Direction::Left }]))),
                        '>' => new_row.push(CellContents::Blizzards(Vec::from([Blizzard { coord: coord, direction: Direction::Right }]))),
                        '.' => { 
                            if !loaded { loaded = true; start_pos = coord; } 
                            end_pos = coord;
                            new_row.push(CellContents::Blizzards(Vec::new())) 
                        },
                         _  => panic!(""),
                    };
                }

                grid.push(new_row);
            }
        }
    }

    let mut moves = 0;
    let mut poss_positions = HashSet::from([start_pos]);
    let mut reached_end = false;
    let mut reached_start = false;
    loop {
        let mut new_grid = grid.clone().into_iter().map(|row| 
            row.into_iter().map (|cell| 
                match cell {
                    CellContents::Wall => CellContents::Wall,
                    CellContents::Blizzards(_) => CellContents::Blizzards(Vec::new()),
                }
            ).collect::<Vec<CellContents>>()
        ).collect::<Vec<Vec<CellContents>>>();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                match &grid[row][col] {
                    CellContents::Wall => {},
                    CellContents::Blizzards(blizs) => {
                        for bliz in blizs {
                            let next_coord = match bliz.direction {
                                Direction::Up => match grid[row - 1][col] {
                                    CellContents::Wall => Coord{ x: col, y: grid.len() - 2 },
                                    CellContents::Blizzards(_) => Coord { x: col, y: row - 1 },
                                },
                                Direction::Down => match grid[row + 1][col] {
                                    CellContents::Wall => Coord{ x: col, y: 1 },
                                    CellContents::Blizzards(_) => Coord { x: col, y: row + 1 },
                                },
                                Direction::Left => match grid[row][col - 1] {
                                    CellContents::Wall => Coord{ x: grid[0].len() - 2, y: row },
                                    CellContents::Blizzards(_) => Coord { x: col - 1, y: row },
                                },
                                Direction::Right => match grid[row][col + 1] {
                                    CellContents::Wall => Coord{ x: 1, y: row },
                                    CellContents::Blizzards(_) => Coord { x: col + 1, y: row },
                                },
                            };

                            let mut new_bliz = bliz.clone();
                            new_bliz.coord = next_coord;

                            match &mut new_grid[next_coord.y][next_coord.x] {
                                CellContents::Blizzards(blizs) => blizs.push(new_bliz),
                                _ => panic!(""),
                            };
                        }
                    }
                }
            }
        }

        grid = new_grid;

        let mut new_positions = HashSet::new();
        for pos in poss_positions {
            let mut move_opts = Vec::from([(pos.x, pos.y), (pos.x - 1, pos.y), (pos.x + 1, pos.y)]);

            // don't go down to start with, or up from the end
            if pos.y > 0 { move_opts.push((pos.x, pos.y - 1)); }
            if pos.y < grid.len() - 1 { move_opts.push((pos.x, pos.y + 1)); }

            for (new_x, new_y) in move_opts {
                match &grid[new_y][new_x] {
                    CellContents::Wall => {},
                    CellContents::Blizzards(blizs) => if blizs.len() == 0 { 
                        new_positions.insert(Coord { x: new_x, y: new_y });
                    },
                }
            }
        }

        poss_positions = new_positions;
        moves += 1;

        if !reached_end && poss_positions.contains(&end_pos) {
            println!("Reached end after {moves} moves");
            reached_end = true;
            poss_positions = HashSet::from([end_pos]);
        } else if !reached_start && reached_end && poss_positions.contains(&start_pos) {
            println!("Reached start again after {moves} moves");
            reached_start = true;
            poss_positions = HashSet::from([start_pos]);
        } else if reached_start && poss_positions.contains(&end_pos) {
            println!("Got back with the snacks after {moves} moves");
            return;
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

fn print_grid(grid: &Vec<Vec<CellContents>>, player_pos: Coord) {
    for (row_num, row) in grid.into_iter().enumerate() {
        for (col, cell) in row.into_iter().enumerate() {
            if (player_pos.x, player_pos.y) == (col, row_num) {
                print!("P");
                continue;
            }

            match cell {
                CellContents::Wall => print!("#"),
                CellContents::Blizzards(blizs) => {
                    match blizs.len() {
                        0 => print!("."),
                        1 => match blizs[0].direction {
                            Direction::Up => print!("^"),
                            Direction::Down => print!("v"),
                            Direction::Left => print!("<"),
                            Direction::Right => print!(">"),
                        },
                        _ => print!("{}", blizs.len() % 10),
                    }
                },
            }
        }
        println!("");
    }
    println!("");
}

