use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coord {
    x: i64,
    y: i64
}

enum Move {
    North = 0, South = 1, West = 2, East = 3
}

fn main() {
    let mut elves = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.into_iter().enumerate() {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                for (column, letter) in line.chars().enumerate() {
                    match letter {
                        '.' => {},
                        '#' => elves.push(Coord { x: column as i64, y: line_num as i64 }),
                         _  => panic!(""),
                    }
                }
            }
        }
    }

    fn check_no_elves_adjacent(elves: &Vec<Coord>, elf: Coord) -> bool {
        for x in (elf.x - 1)..=(elf.x + 1) {
            for y in (elf.y - 1)..=(elf.y + 1) {
                if x == elf.x && y == elf.y { continue; }
                if elves.contains(&Coord { x: x, y: y }) {
                    return false;
                }
            }
        }
        return true;
    }

    fn calc_proposed(move_index: usize, elves: &Vec<Coord>, elf: Coord) -> (bool, Coord) {
        let mut next_move_dir = match move_index % 4 {
            0 => Move::North, 1 => Move::South, 2 => Move::West, 3 => Move::East, _ => panic!("")
        };

        let mut moves = Vec::new();
        for i in 0..4 {
            let new_move = match next_move_dir {
                Move::North => [(elf.x, elf.y - 1), (elf.x + 1, elf.y - 1), (elf.x - 1, elf.y - 1)],
                Move::South => [(elf.x, elf.y + 1), (elf.x + 1, elf.y + 1), (elf.x - 1, elf.y + 1)],
                Move::West =>  [(elf.x - 1, elf.y), (elf.x - 1, elf.y - 1), (elf.x - 1, elf.y + 1)],
                Move::East =>  [(elf.x + 1, elf.y), (elf.x + 1, elf.y - 1), (elf.x + 1, elf.y + 1)],
            };
            moves.push(new_move);

            next_move_dir = match next_move_dir {
                Move::North => Move::South, Move::South => Move::West,
                Move::West =>  Move::East,  Move::East =>  Move::North
            };
        }

        for _move in moves {
            if _move.iter().fold(true, |free, (x,y)| free && !elves.contains(&Coord { x: *x, y: *y })) {
                return (true, Coord { x: _move[0].0, y: _move[0].1 });
            }
        }
        // println!("no valid moves for {:?}", elf);
        return (false, elf);
    }

    fn print_grid(elves: &Vec<Coord>) {
        let (min_x, max_x, min_y, max_y) = elves.iter().fold((1_000_000, -1_000_000, 1_000_000, -1_000_000), |min_max, pos| {
            // println!("min_max: {:?}, pos: {:?}", min_max, pos);
            let mut res = min_max;
            if pos.x < res.0 { res.0 = pos.x; }
            if pos.x > res.1 { res.1 = pos.x; }
            if pos.y < res.2 { res.2 = pos.y; }
            if pos.y > res.3 { res.3 = pos.y; }
            return res;
        });

        // println!("{:?}", elves);
        print!(" ");
        for col in min_x..=max_x {
            print!("{}", (col % 10).abs());
        }
        println!("");

        let mut empties = 0;
        for row in min_y..=max_y {
            print!("{}", (row % 10).abs());
            for col in min_x..=max_x {
                if elves.contains(&Coord {x: col, y: row}) { 
                    print!("#");
                } else { 
                    print!(".");
                    empties += 1;
                }
            }
            println!("");
        }
        println!("");

        println!("origin: {:?}, end: {:?}, Grid size: {}x{} = {}", 
            (min_x, min_y), (max_x, max_y),
            max_x - min_x + 1, max_y - min_y + 1, (max_x - min_x + 1) * (max_y - min_y + 1));
        println!("empties: {empties}");
        // let mut sorted = elves.clone();
        // sorted.sort_by(|a,b| a.y.partial_cmp(&b.y).unwrap());
        // println!("{:?}", sorted);
    }
 
    print_grid(&elves);

    for round in 0..10 {
        let mut proposed = Vec::new();
        let mut moves = HashSet::new();
        let mut clashes = HashSet::new();
        for elf in &elves {
            let mut next_move = *elf;
            if !check_no_elves_adjacent(&elves, *elf) {
                let (valid, next) = calc_proposed(round, &elves, *elf);
                if valid { next_move = next; }
            }

            proposed.push(next_move);
            if !moves.insert(next_move) {
                clashes.insert(next_move);
            }
        }

        let mut final_pos = Vec::new();
        for (index, elf) in elves.iter().enumerate() {
            let next = proposed[index];
            if clashes.contains(&next) {
                final_pos.push(*elf);
            } else {
                final_pos.push(next);
            }
        }

        // println!("{:?}", (min_x, max_x, min_y, max_y));
        elves = final_pos;
    }

    print_grid(&elves);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
