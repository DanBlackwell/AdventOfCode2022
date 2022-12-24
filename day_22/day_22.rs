use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Nonexistant,
    Clear,
    Wall
}

#[derive(Debug)]
enum Instr {
    Ahead(usize),
    Left, 
    Right
}

#[derive(Debug, Clone, Copy)]
enum Heading { Up = 3, Down = 1, Left = 2, Right = 0 }

#[derive(Debug, Clone, Copy)]
enum Rotation { Left_90, Right_90, UpsideDown, Same }

struct Face {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    neighbour_up: (usize, Rotation),
    neighbour_down: (usize, Rotation),
    neighbour_left: (usize, Rotation),
    neighbour_right: (usize, Rotation),
}

fn main() {
    let mut grid = Vec::new();
    let mut hit_empty = false;
    let mut instructions = Vec::new();
    let mut widest = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { hit_empty = true; continue; }

                if hit_empty {
                    let mut num: usize = 0;
                    for letter in line.chars() {
                        match letter {
                            '0'..='9' => num = num * 10 + (letter as u8 - '0' as u8) as usize,
                            'L' => { instructions.push(Instr::Ahead(num)); instructions.push(Instr::Left); num = 0; },
                            'R' => { instructions.push(Instr::Ahead(num)); instructions.push(Instr::Right); num = 0; },
                             _  => panic!(),
                        }
                    }

                    instructions.push(Instr::Ahead(num));
                } else {
                    let mut new_row = line.chars().map(|c| 
                        match c {
                            ' ' => Tile::Nonexistant,
                            '.' => Tile::Clear,
                            '#' => Tile::Wall,
                             _  => panic!(),
                        })
                        .collect::<Vec<Tile>>();

                    if new_row.len() > widest { widest = new_row.len(); }
                    grid.push(new_row);
                }
            }
        }
    }

    let mut temp = Vec::new();
    for (row_num, row) in grid.into_iter().enumerate() {
        let row_len = row.len();
        if row_len < widest {
            let mut new_row = row.clone();
            for _ in 0..(widest - row_len) { new_row.push(Tile::Nonexistant); }
            temp.push(new_row);
        } else {
            temp.push(row.clone());
        }
    }
    grid = temp;

    let mut heading = Heading::Right;
    let mut row_col = (0,0);
    let mut found = false;
    'outer: for (row_num, row) in grid.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            if *col == Tile::Clear { 
                row_col = (row_num, col_num);
                found = true;
                break 'outer;
            }
        }
    }
    if !found { panic!(""); }

    fn turn_left(heading: Heading) -> Heading {
        match heading {
            Heading::Up => Heading::Left,
            Heading::Left => Heading::Down,
            Heading::Down => Heading::Right,
            Heading::Right => Heading::Up,
        }
    }

    fn turn_right(heading: Heading) -> Heading { 
        match heading {
            Heading::Up => Heading::Right,
            Heading::Right => Heading::Down,
            Heading::Down => Heading::Left,
            Heading::Left => Heading::Up,
        }
    }

    // return false if hit wall
    fn move_one_part_one(grid: &Vec<Vec<Tile>>, row_col: &mut (usize, usize), heading: Heading) -> bool {
        let (row, col) = row_col;
        match heading {
            Heading::Up => if *row == 0 || grid[*row - 1][*col] == Tile::Nonexistant {
                for index in (0..grid.len()).rev() {
                    match grid[index][*col] {
                        Tile::Wall => return false,
                        Tile::Clear => { row_col.0 = index; return true; },
                        Tile::Nonexistant => {},
                    }
                }
                panic!();
            } else if grid[*row - 1][*col] == Tile::Wall {
                // do nothing
                return false;
            } else {
                row_col.0 = *row - 1;
            } ,
            Heading::Down => if *row == grid.len() - 1 || grid[*row + 1][*col] == Tile::Nonexistant {
                for index in 0..grid.len() {
                    match grid[index][*col] {
                        Tile::Wall => return false,
                        Tile::Clear => { row_col.0 = index; return true; },
                        Tile::Nonexistant => {},
                    }
                }
                panic!();
            } else if grid[*row + 1][*col] == Tile::Wall {
                // do nothing
                return false;
            } else {
                row_col.0 = *row + 1;
            } ,
            Heading::Left => if *col == 0 || grid[*row][*col - 1] == Tile::Nonexistant {
                for index in (0..grid[0].len()).rev() {
                    match grid[*row][index] {
                        Tile::Wall => return false,
                        Tile::Clear => { row_col.1 = index; return true; },
                        Tile::Nonexistant => {},
                    }
                }
                panic!();
            } else if grid[*row][*col - 1] == Tile::Wall {
                // do nothing
                return false;
            } else {
                row_col.1 = *col - 1;
            } ,
            Heading::Right => if *col == grid[*row].len() - 1 || grid[*row][*col + 1] == Tile::Nonexistant {
                for index in 0..grid[0].len() {
                    match grid[*row][index] {
                        Tile::Wall => return false,
                        Tile::Clear => { row_col.1 = index; return true; },
                        Tile::Nonexistant => {},
                    }
                }
                panic!();
            } else if grid[*row][*col + 1] == Tile::Wall {
                // do nothing
                return false;
            } else {
                row_col.1 = *col + 1;
            } ,
        };

        return true;
    }

    fn print_grid(grid: &Vec<Vec<Tile>>, row_col: (usize, usize), heading: Heading) {
        for (row_num, row) in grid.into_iter().enumerate() {
            for (col_num, col) in row.into_iter().enumerate() {
                print!("{}", 
                    if (row_num, col_num) == row_col {
                        match heading { Heading::Up => "^", Heading::Down => "v", 
                                        Heading::Left => "<", Heading::Right => ">" }
                    }
                    else if *col == Tile::Nonexistant { " " } 
                    else if *col == Tile::Clear { "." } 
                    else { "#" }
                );
            }
            println!("");
        }
        println!("");
    }

    let mut row_col_part_2 = row_col;

    let mut faces = Vec::new();



    'instr_loop: for instr in instructions.iter() {
        match instr {
            Instr::Left => heading = turn_left(heading),
            Instr::Right => heading = turn_right(heading),
            Instr::Ahead(distance) => {
                // println!("Moving forward {distance}");
                for i in 0..*distance {
                    let success = move_one_part_one(&grid, &mut row_col, heading);
                    // if !success { 
                    //     // println!("Hit a wall after {i} moves");
                    //     continue 'outer; 
                    // }
                    // print_grid(&grid, row_col, heading);
                }
            },
        }
        // print_grid(&grid, row_col, heading);
    }

    // println!("{:?}", instructions);

    println!("final row: {}, col: {}, heading: {:?}", row_col.0 + 1, row_col.1 + 1, heading);
    println!("sum: {}", (row_col.0 + 1) * 1000 + (row_col.1 + 1) * 4 + heading as usize);
}

fn get_faces(grid: &Vec<Vec<Tile>>) -> Vec<Face> {
    let mut faces = Vec::new();
   faces.push(Face { 
        min_x: (2 * grid[0].len()) / 4,
        max_x: (3 * grid[0].len()) / 4 - 1,
        min_y: 0,
        min_y: (1 * grid.len()) / 3 - 1,
        neighbour_up: (1, Rotation::Same),
        neighbour_down: (3, Rotation::Same),
        neighbour_left: (2, Rotation::Right_90),
        neighbour_right: (5, Rotation::UpsideDown),
    });
    faces.push(Face { 
        min_x: (0 * grid[0].len()) / 4,
        max_x: (1 * grid[0].len()) / 4 - 1,
        min_y: (1 * grid.len()) / 3,
        min_y: (2 * grid.len()) / 3 - 1,
        neighbour_up: (4, Rotation::UpsideDown),
        neighbour_down: (0, Rotation::UpsideDown),
        neighbour_left: (5, Rotation::Left_90),
        neighbour_right: (2, Rotation::Same),
    });
    faces.push(Face { 
        min_x: (1 * grid[0].len()) / 4,
        max_x: (2 * grid[0].len()) / 4 - 1,
        min_y: (1 * grid.len()) / 3,
        min_y: (2 * grid.len()) / 3 - 1,
        neighbour_up: (0, Rotation::Right_90),
        neighbour_down: (4, Rotation::Left_90),
        neighbour_left: (1, Rotation::Same),
        neighbour_right: (3, Rotation::Same),
    });
    faces.push(Face { 
        min_x: (2 * grid[0].len()) / 4,
        max_x: (3 * grid[0].len()) / 4 - 1,
        min_y: (1 * grid.len()) / 3,
        min_y: (2 * grid.len()) / 3 - 1,
        neighbour_up: (0, Rotation::Same),
        neighbour_down: (4, Rotation::Same),
        neighbour_left: (5, Rotation::Right_90),
        neighbour_right: (2, Rotation::Same),
    });
    faces.push(Face { 
        min_x: (2 * grid[0].len()) / 4,
        max_x: (3 * grid[0].len()) / 4 - 1,
        min_y: (2 * grid.len()) / 3,
        min_y: (3 * grid.len()) / 3 - 1,
        neighbour_up: (3, Rotation::Same),
        neighbour_down: (1, Rotation::UpsideDown),
        neighbour_left: (2, Rotation::Left_90),
        neighbour_right: (5, Rotation::Same),
    });
    faces.push(Face { 
        min_x: (3 * grid[0].len()) / 4,
        max_x: (4 * grid[0].len()) / 4 - 1,
        min_y: (2 * grid.len()) / 3,
        min_y: (3 * grid.len()) / 3 - 1,
        neighbour_up: (3, Rotation::Left_90),
        neighbour_down: (1, Rotation::Left_90),
        neighbour_left: (4, Rotation::Same),
        neighbour_right: (0, Rotation::UpsideDown),
    }); 
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
