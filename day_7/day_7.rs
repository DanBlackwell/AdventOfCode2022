use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct Arena {
    nodes: Vec<Node>,
}

struct Node {
    name: String,
    parent: Option<usize>,
    subdirs: Vec<usize>,
    files: HashMap<String, usize>,
    files_size: usize,
}

fn new_node(arena: &mut Arena, name: String, parent: Option<usize>) -> usize {
    let next_index = arena.nodes.len();
    
    arena.nodes.push(Node {
        name: name,
        parent: parent, 
        subdirs: Vec::new(),
        files: HashMap::new(),
        files_size: 0,
    });
    
    return next_index;
}

fn main() {
    let mut arena = Arena { nodes: Vec::new() };
    let root = new_node(&mut arena, String::from("root"), None);
    let mut cur_node = root;
    
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                if line.starts_with("$ cd ..") {
                    if let Some(node) = arena.nodes[cur_node].parent {
                        cur_node = node;
                    }
                } else if line.starts_with("$ cd ") {
                    let (_, dirname) = line.split_at(5);
                    
                    let mut found = false;
                    for node in arena.nodes[cur_node].subdirs.iter() {
                        if arena.nodes[*node].name == dirname.to_string() {
                            cur_node = *node;
                            found = true;
                            break;
                        }
                    }
                    
                    if !found {
                        let new_node = new_node(&mut arena, dirname.to_string(), Some(cur_node));
                        arena.nodes[cur_node].subdirs.push(new_node);
                        cur_node = new_node;
                        // println!("node {cur_node}: Added new child {new_node}");
                    }
                } else if line.starts_with("$ ls") {
                    // boring, nothing to do
                } else if line.starts_with("dir ") {
                    let (_, dirname) = line.split_at(4);
                    
                    let mut found = false;
                    for node in arena.nodes[cur_node].subdirs.iter() {
                        if arena.nodes[*node].name == dirname.to_string() {
                            found = true;
                            break;
                        }
                    }
                    
                    if !found {
                        let new_node = new_node(&mut arena, dirname.to_string(), Some(cur_node));
                        arena.nodes[cur_node].subdirs.push(new_node);
                        // println!("node {cur_node}: Added new child {new_node}");
                    }
                    
                } else /* size_bytes filename */ {
                
                    let vals = line.split(' ').collect::<Vec<&str>>();
                    if let Ok(filesize) = vals[0].parse::<usize>() {
                        arena.nodes[cur_node].files.insert(vals[1].to_string(), filesize);
                        arena.nodes[cur_node].files_size += filesize;
                        // println!("node {cur_node} file: {} {filesize}B", vals[1]);
                    }
                    
                }
            }
        }
    }
    
    let (total_size, sum_below_100k) = sum_dirs_below_100k(&arena, root);
    println!("Sum of those dirs below 100kb {sum_below_100k}");
    let free_disk_space = 70_000_000 - total_size;
    let required_deletion_size = 30_000_000 - free_disk_space;
    println!("need to free up {required_deletion_size}");
    let (_, smallest) = smallest_dir_to_delete(&arena, root, required_deletion_size);
    println!("Smallest dir to delete: {smallest} bytes");
}

fn sum_dirs_below_100k(arena: &Arena, dir_node: usize) -> (usize, usize) {
    let mut total_size = 0;
    let mut sum_below_100k = 0;
    
    for subdir in arena.nodes[dir_node].subdirs.iter() {
        let (temp_total, temp_sum) = sum_dirs_below_100k(arena, *subdir);
        total_size += temp_total;
        sum_below_100k += temp_sum;
    }
    
    total_size += arena.nodes[dir_node].files_size;
    
    if total_size <= 100000 {
        sum_below_100k += total_size;
    }
    
    return (total_size, sum_below_100k);
}


fn smallest_dir_to_delete(arena: &Arena, dir_node: usize, required_space: usize) -> (usize, usize) {
    let mut smallest_deletion = 100_000_000;
    let mut total_size = 0;
    
    for subdir in arena.nodes[dir_node].subdirs.iter() {
        let (temp_total, temp_smallest) = smallest_dir_to_delete(arena, *subdir, required_space);
        total_size += temp_total;
        if temp_smallest >= required_space && temp_smallest < smallest_deletion { 
            smallest_deletion = temp_smallest; 
        }
    }
    
    total_size += arena.nodes[dir_node].files_size;
    
    if total_size >= required_space && total_size < smallest_deletion {
        smallest_deletion = total_size;
    }
    
    return (total_size, smallest_deletion);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
