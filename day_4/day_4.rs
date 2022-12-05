use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let mut fully_redundant = 0;
  let mut partial_redundant = 0;

  if let Ok(lines) = read_lines("input.txt") {
    for line in lines {
      if let Ok(line) = line {
        let mut elf_ranges: [[u32; 2]; 2] = [[0; 2]; 2];

        for (elf_index, zone) in line.split(",").enumerate() {
          for (index, part) in zone.split("-").enumerate() {
            let val = part.parse::<u32>().unwrap();
            elf_ranges[elf_index][index] = val;
          }
        }
      
        if (elf_ranges[0][0] <= elf_ranges[1][0] && elf_ranges[0][1] >= elf_ranges[1][1]) ||
           (elf_ranges[1][0] <= elf_ranges[0][0] && elf_ranges[1][1] >= elf_ranges[0][1]) {
          fully_redundant += 1;
          partial_redundant += 1;
          println!("Full overlap for {line}");
        } else if (elf_ranges[0][0] >= elf_ranges[1][0] && elf_ranges[0][0] <= elf_ranges[1][0]) ||
                  (elf_ranges[0][1] >= elf_ranges[1][0] && elf_ranges[0][1] <= elf_ranges[1][1]) ||
                  (elf_ranges[1][0] >= elf_ranges[0][0] && elf_ranges[1][0] <= elf_ranges[0][1]) ||
                  (elf_ranges[1][1] >= elf_ranges[0][0] && elf_ranges[1][1] <= elf_ranges[0][1]) {
          partial_redundant += 1;
          println!("Partial overlap for {line}");
        }
      }
    }
  }

  println!("Total fully redundant: {fully_redundant}, partially redundant: {partial_redundant}"); 
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

