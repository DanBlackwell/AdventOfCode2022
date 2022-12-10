use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn check(signal_strength_cycles: &mut Vec<i32>, cur_cycle: i32, total_signal_strength: &mut i32, register_value: i32, output_row: &mut Vec<char>) {
  if let Some(sig_cycle) = signal_strength_cycles.first() {
    if cur_cycle == *sig_cycle {
      *total_signal_strength += cur_cycle * register_value;
      signal_strength_cycles.drain(0..1);
    }
  }

  output_row.push(
    if register_value - 1 <= (cur_cycle - 1) % 40 && register_value + 1 >= (cur_cycle - 1) % 40 { '#' } else { '.' }
  );
  if cur_cycle % 40 == 0 {
    println!("{}", output_row.iter().collect::<String>());
    output_row.clear();
  }
}

fn main() {
    let mut signal_strength_cycles: Vec<i32> = Vec::from([20, 60, 100, 140, 180, 220]);
    let mut cur_cycle: i32 = 1;
    let mut total_signal_strength: i32 = 0;
    let mut register_value: i32 = 1;

    let mut output_row: Vec<char> = Vec::new();

    check(&mut signal_strength_cycles, cur_cycle, &mut total_signal_strength, register_value, &mut output_row);

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }

                if line == "noop" {
                  cur_cycle += 1;
                  check(&mut signal_strength_cycles, cur_cycle, &mut total_signal_strength, register_value, &mut output_row);
                } else {
                  let (_, num) = line.split_at(5);
                  let add_val = num.parse::<i32>().unwrap();

                  cur_cycle += 1;
                  check(&mut signal_strength_cycles, cur_cycle, &mut total_signal_strength, register_value, &mut output_row);

                  register_value += add_val;
                  cur_cycle += 1;
                  check(&mut signal_strength_cycles, cur_cycle, &mut total_signal_strength, register_value, &mut output_row);
                }
            }
        }
    }

    println!("total signal strength: {total_signal_strength}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
