use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
  let mut total = 0;
  let mut group_count = 0;
  let mut group_letters: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];

  if let Ok(lines) = read_lines("input.txt") {
    for line in lines {
      if let Ok(ruck_conts) = line {
        if ruck_conts.len() == 0 { continue; }

        let compartment_1 = &ruck_conts[0..(ruck_conts.len() / 2)];
        let comp_1_set: HashSet<char> = HashSet::from_iter(compartment_1.chars());

        let compartment_2 = &ruck_conts[(ruck_conts.len() / 2)..];
        let comp_2_set: HashSet<char> = HashSet::<char>::from_iter(compartment_2.chars());

//        let duplicates = comp_1_set.intersection(&comp_2_set).copied();
//        let mut duped = ' ';
//        for val in duplicates {
//          duped = val;
//        }

        group_letters[group_count] = HashSet::from_iter(ruck_conts.chars());

        group_count += 1;
        if group_count == 3 {
          let intersection1 = group_letters[0].intersection(&group_letters[1]).copied();
          let set1 = HashSet::from_iter(intersection1);
          let intersection2 = set1.intersection(&group_letters[2]).copied();
          let mut badge = ' ';
          for val in intersection2 {
            println!("Intersection between group: {val}");
            badge = val;
          }
          
          total += value_for_letter(badge);

          group_count = 0;
        }
      }
    }
  } 

  println!("Final result: {total}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn value_for_letter(letter: char) -> u32 {
  const LOWER_A: char = 'a';
  const LOWER_Z: char = 'z';
  const UPPER_A: char = 'A';
  const UPPER_Z: char = 'Z';

  if letter >= LOWER_A && letter <= LOWER_Z {
    return 1 + letter as u32 - LOWER_A as u32;
  } else if letter >= UPPER_A && letter <= UPPER_Z {
    return 26 + 1 + letter as u32 - UPPER_A as u32;
  } else {
    return 0xbadf00d;
  }
}
