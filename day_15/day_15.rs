use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut sensor_beacon: Vec<((i32, i32), (i32, i32))> = Vec::new();
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let re = Regex::new(r"Sensor at x=(\-?[0-9]+), y=(\-?[0-9]+): closest beacon is at x=(\-?[0-9]+), y=(\-?[0-9]+)");
                for cap in re.expect("HELP").captures_iter(line.as_str()) {
                    sensor_beacon.push((
                        (cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()), 
                        (cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap())
                    ));
                }
            }
        }
    }
    
    const TARGET_ROW: i32 = 2_000_000;
    const PART_2_DIM: i32 = 2 * TARGET_ROW;
    let mut target_row_filled: HashSet<i32> = HashSet::new();
    let beacons: HashSet<(i32, i32)> = HashSet::from_iter(sensor_beacon.clone().into_iter().map(|(_,b)| b).into_iter());
    
    let mut missing_beacon_ranges: Vec<Vec<std::ops::RangeInclusive<i32>>> = Vec::new();
    for _row in 0..=PART_2_DIM { missing_beacon_ranges.push(Vec::new()); }
    
    for (sensor, beacon) in sensor_beacon {
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        
        // Part 2
        let range;
        if sensor.1 > PART_2_DIM && sensor.1 - distance <= PART_2_DIM {
            range = (sensor.1 - distance)..=PART_2_DIM;
        } else if sensor.1 < 0 && sensor.1 + distance >= 0 {
            range = 0..=(sensor.1 + distance);
        } else {
            range = std::cmp::max(0, sensor.1 - distance)..=std::cmp::min(sensor.1 + distance, PART_2_DIM);
        }
        
        for row in range {
            let l_r_dist = distance - (row - sensor.1).abs();
            let x_range;
            if sensor.0 > PART_2_DIM && sensor.0 - l_r_dist <= PART_2_DIM {
                x_range = (sensor.0 - l_r_dist)..=PART_2_DIM;
            } else if sensor.0 < 0 && sensor.0 + l_r_dist >= 0 {
                x_range = 0..=(sensor.0 + l_r_dist);
            } else {
                x_range = std::cmp::max(0, sensor.0 - l_r_dist)..=std::cmp::min(sensor.0 + l_r_dist, PART_2_DIM);
            }
            
            missing_beacon_ranges[row as usize].push(x_range);
        }
        
        // Part 1
        let left_right_dist: i32;
        if sensor.1 <= TARGET_ROW && sensor.1 + distance >= TARGET_ROW {
            left_right_dist = (sensor.1 + distance) - TARGET_ROW;
        } else if sensor.1 >= TARGET_ROW && sensor.1 - distance <= TARGET_ROW { 
            left_right_dist = TARGET_ROW - (sensor.1 - distance);
        } else {
            continue;
        }
        
        // this ought to be done using ranges, similarly to part 2
        for i in -left_right_dist..=left_right_dist {
            if !beacons.contains(&(sensor.0 + i, TARGET_ROW)) {
                target_row_filled.insert(sensor.0 + i);
            }
        }
    }
    
    println!("total filled: {}", target_row_filled.len());
    
    for row in 0..=PART_2_DIM {
    
        missing_beacon_ranges[row as usize].sort_by(|a, b| a.start().partial_cmp(b.start()).unwrap());
        let mut consolidated: Vec<std::ops::RangeInclusive<i32>> = Vec::new();
        let mut prev = missing_beacon_ranges[row as usize][0].clone();
        
        for i in 1..missing_beacon_ranges[row as usize].len() {
            let next = &missing_beacon_ranges[row as usize][i];
            if &(prev.end() + 1) >= next.start() {
                prev = *prev.start()..=std::cmp::max(*prev.end(), *next.end());
            } else {
                consolidated.push(prev.clone());
                prev = next.clone();
            }
        }
        consolidated.push(prev);
        
        if consolidated.len() != 1 {
            println!("part 2: {:?}, tuning frequency: {}", 
                (consolidated[0].end() + 1, row), 
                (4_000_000 * (consolidated[0].end() + 1) as usize) + row as usize
            );
            break;
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


        
