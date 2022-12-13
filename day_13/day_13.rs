use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use serde_json::Value;
use serde_json::json;

fn check_in_order(json_1: &Value, json_2: &Value) -> i32 {
    let array_1: &Vec<Value>;
    let array_2: &Vec<Value>;
    let temp: Vec<Value>;
    
    match json_1 {
        Value::Array(arr_1) =>
            match json_2 {
                Value::Array(arr_2) => {
                    array_1 = arr_1;
                    array_2 = arr_2;
                },
                Value::Number(_) => {
                    array_1 = arr_1;
                    temp = Vec::from([json_2.clone()]); 
                    // println!("Created array for 2: {:?}", temp);
                    array_2 = &temp;
                },
                _ => panic!(),
            },
        Value::Number(num_1) =>
            match json_2 {
                Value::Number(num_2) => {
                    // println!("Comparing: {} < {}", num_1.as_i64().unwrap(), num_2.as_i64().unwrap());
                    let n1 = num_1.as_i64().unwrap();
                    let n2 = num_2.as_i64().unwrap();
                    if n1 < n2 {
                        // println!("result 1");
                        return 1;
                    } else if n1 > n2 {
                        // println!("result -1");
                        return -1;
                    } else {
                        // println!("result 0");
                        return 0;
                    }
                },
                Value::Array(arr_2) => {
                        temp = Vec::from([json_1.clone()]); 
                        // println!("Created array for 1: {:?}", temp);
                        array_1 = &temp;
                        array_2 = arr_2;
                    },
                _ => {
                    println!("Expected {} and {} to be numbers!", json_1.to_string(), json_2.to_string());
                    panic!();
                },
            },
        _ => panic!(),
    }
    
    // println!("array_1: {:?}\narray_2: {:?}", array_1, array_2);
    if array_2.len() == 0 && array_1.len() > 0 { return -1; }
    for index in 0..std::cmp::max(array_1.len(), array_2.len()) {
        if index >= array_1.len() { return 1; }
        if index >= array_2.len() { return -1; }
        // println!("Comparing values: {:?} to {:?}", array_1[index], array_2[index]);
        
        let order = check_in_order(&array_1[index], &array_2[index]);
        if order == 1 {
            // println!("Were in order, bailing");
            return 1;
        } else if order == -1 {
            // println!("Out of order, ret -1");
            return -1;
        }
    }
    
    // println!("Iterated whole array, bailing");
    return 0;
}

fn main() {
    let mut all_packets = Vec::new();
    let mut read_line_1 = false;
    let mut json_1: Value = serde_json::from_str("\"test\"").unwrap();
    let mut count = 1;
    let mut in_order_total = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                all_packets.push(serde_json::from_str(line.as_str()).unwrap());
                
                if !read_line_1 {
                    json_1 = serde_json::from_str(line.as_str()).unwrap();
                    read_line_1 = true;
                } else {
                    let json_2: Value = serde_json::from_str(line.as_str()).unwrap();
                    
                    let in_order = check_in_order(&json_1, &json_2);
                    // println!("{count}: {}", if in_order >= 0 { "in order" } else { "NOT in order" });
                    if in_order > 0 { in_order_total += count; }
                    count += 1;
                            
                    read_line_1 = false;
                }
                
            }
        }
    }
    
    println!("In order count: {in_order_total}");
    let divider_1 = json!([[2]]);
    let divider_2 = json!([[6]]);
    all_packets.push(divider_1.clone());
    all_packets.push(divider_2.clone());
    
    all_packets.sort_by(|a, b| { 
        let res = check_in_order(a, b);
        match res {
          -1 => return std::cmp::Ordering::Greater,
           1 => return std::cmp::Ordering::Less,
           _ => return std::cmp::Ordering::Equal,
        };
    });
    
    // println!("sorted: ");
    // for pkt in all_packets.iter() {
    //     println!("{}", pkt.to_string());
    // }
    
    let pos_1 = all_packets.iter().position(|x| *x == divider_1).unwrap();
    let pos_2 = all_packets.iter().position(|x| *x == divider_2).unwrap();
    println!("part 2: {}", (pos_1 + 1) * (pos_2 + 1));
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
