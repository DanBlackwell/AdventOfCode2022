use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let mut initial_array = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                initial_array.push(line.parse::<i64>().unwrap());
            }
        }
    }

    println!("initial_array: {:?}", initial_array);

    // let mut start_to_cur_pos = initial_array.clone().into_iter()
    //     .enumerate()
    //     .fold(HashMap::new(), |mut map, (idx, x)| { map.insert(idx, idx); return map; });
    let mut start_to_cur_pos = Vec::new();
    for i in 0..initial_array.len() { start_to_cur_pos.push(i); }
    // println!("start_to_cur_pos: {:?}", start_to_cur_pos);

    let len = initial_array.len();
    for (index, value) in initial_array.iter().enumerate() {
        let mov_dist = value % (len - 1) as i64;
        let cur_pos = start_to_cur_pos[index];
        let mut insert_index;

        if mov_dist < 0 {
            // println!("mov_dist < 0");
            // println!("cur_pos: {cur_pos}, mov_dist: {mov_dist}");
            if (cur_pos as i64) <= -mov_dist {
                // println!("cur_pos < -mov_dist, cur_pos - mov_dist: {:?}", (cur_pos as i64) - mov_dist);
                // println!("len: {len}");
                insert_index = len - 1 - ((cur_pos as i64) + mov_dist).abs() as usize;
                // println!("insert_index: {insert_index}");
            } else {
                // println!("cur_pos >= -mov_dist");
                insert_index = ((cur_pos as i64) + mov_dist) as usize;
            }
        } else {
            if cur_pos + (mov_dist as usize) > len {
                insert_index = ((cur_pos as i64) + mov_dist) as usize - len + 1;
            } else {
                insert_index = ((cur_pos as i64) + mov_dist) as usize;
            }
        }

        // if mov_dist <= cur_pos
        // if mov_dist <= -len || insert_index == 0 && cur_pos != 0 { insert_index = len - 1; }
        // else if insert_index == len - 1 && cur_pos != len - 1 { insert_index = 0; }
        // println!("Moving value: {value} from index {cur_pos} to {insert_index}");
        // println!("{:?}", start_to_cur_pos);

        start_to_cur_pos = start_to_cur_pos
            .iter()
            .map(|&index| 
                    if index == cur_pos {
                        insert_index
                    } else if insert_index > cur_pos {
                        // println!("moved to later in the array index: {index}, insert_index: {insert_index}, cur_pos: {cur_pos}");
                        if index <= insert_index && index > cur_pos { index - 1 }  
                        else { index }
                    } else if insert_index < cur_pos {
                        // println!("moved to earlier in the array");
                        if index >= insert_index && index < cur_pos { index + 1 }
                        else { index }
                    } else { index }
                )
            .collect::<Vec<usize>>();
        // println!("start_to_cur_pos after {value} (insert_index: {insert_index}): {:?}", start_to_cur_pos);
        // print!("list: [");
        // for i in 0..initial_array.len() {
        //     for (index, j) in start_to_cur_pos.iter().enumerate() {
        //         if *j == i { print!("{}, ", initial_array[index]); }
        //     }
        // }
        // for i in start_to_cur_pos.iter() {
        //     print!("{}, ", initial_array[*i]);
        // }
        // println!("]");
    }

    let mut final_list = Vec::new();
    for i in 0..initial_array.len() {
        for (index, j) in start_to_cur_pos.iter().enumerate() {
            if *j == i { final_list.push(initial_array[index]); }
        }
    }

    let len = final_list.len();
    let mut zero_pos = 0;
    for i in 0..len { if final_list[i] == 0 { zero_pos = i; break; } }
    let thou = final_list[(zero_pos + 1000) % len];
    let two_thou = final_list[(zero_pos + 2000) % len];
    let three_thou = final_list[(zero_pos + 3000) % len];
    println!("1000: {thou}, 2000: {two_thou}, 3000: {three_thou}, sum: {}", thou + two_thou + three_thou);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
