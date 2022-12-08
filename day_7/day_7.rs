use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// struct Node<'a> {
//     name: Box<&'a str>,
//     parent: Option<Box<Node<'a>>>,
//     subdirs: Vec<Box<Node<'a>>>,
//     files: HashMap<String, usize>,
//     size: usize,
// }

struct Arena {
    nodes: Vec<Node>,
}

struct Node {
    name: String,
    parent: Option<usize>,
    subdirs: Vec<usize>,
    files: HashMap<String, usize>,
    size: usize,
}

fn new_node(arena: &mut Arena, name: String, parent: Option<usize>) -> usize {
    let next_index = arena.nodes.len();
    
    arena.nodes.push(Node {
        name: name,
        parent: parent, 
        subdirs: Vec::new(),
        files: HashMap::new(),
        size: 0,
    });
    
    return next_index;
}
    

fn main() {
    const CHALLENGE_PART_2: bool = true;
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
                        println!("node {cur_node}: Added new child {new_node}");
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
                        println!("node {cur_node}: Added new child {new_node}");
                    }
                    
                } else /* size_bytes filename */ {
                
                    let vals = line.split(' ').collect::<Vec<&str>>();
                    if let Ok(filesize) = vals[0].parse::<usize>() {
                        arena.nodes[cur_node].files.insert(vals[1].to_string(), filesize);
                        arena.nodes[cur_node].size += filesize;
                        println!("node {cur_node} file: {} {filesize}B", vals[1]);
                    }
                    
                }
            }
        }
    }
    
    let mut dfs_stack: Vec<usize> = Vec::new();
    cur_node = root;
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
