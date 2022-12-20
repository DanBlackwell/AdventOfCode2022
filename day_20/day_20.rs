use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Node {
    value: i64,
    next: usize,
    prev: usize,
    original_index: usize
}

fn main() {
    let mut arena: Vec<Node> = Vec::new();
    let mut arena_pt_2: Vec<Node> = Vec::new();
    let mut prev = 0;
    let mut initial_array = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let val = line.parse::<i64>().unwrap();

                let arena_size = arena.len();
                if arena_size > 0 {
                    arena[arena_size - 1].next = arena_size;
                    arena_pt_2[arena_size - 1].next = arena_size;
                }

                let mut node = Node {
                    value: val,
                    next: 0,
                    prev: prev,
                    original_index: arena_size
                };
                arena.push(node.clone());

                node.value *= 811589153;
                arena_pt_2.push(node);

                prev = arena_size;

                initial_array.push(val);
            }
        }
    }

    let arena_size = arena.len();
    arena[0].prev = arena_size - 1;
    arena_pt_2[0].prev = arena_size - 1;

    fn decrypt_once(arena: &mut Vec<Node>) {
        let mut index = 0;
        let arena_size = arena.len();
        for i in 0..arena_size {
            while arena[index].original_index != i { index = arena[index].next; }
            let mut cur_node = arena[index].clone();
            let moves = cur_node.value % (arena_size as i64 - 1);
            if moves < 0 {
                for _ in moves..0 {
                    let prev_node_pos = cur_node.prev;
                    let mut prev_node = arena[prev_node_pos].clone();
                    let this_node_pos = prev_node.next;
                    prev_node.next = cur_node.next;
                    cur_node.prev = prev_node.prev;
                    cur_node.next = this_node_pos;
                    prev_node.prev = prev_node_pos;
                    arena[this_node_pos] = prev_node;
                    arena[prev_node_pos] = cur_node.clone();
                }
            } else {
                for _ in 0..moves {
                    let next_node_pos = cur_node.next;
                    let mut next_node = arena[next_node_pos].clone();
                    let this_node_pos = next_node.prev;
                    next_node.prev = cur_node.prev;
                    cur_node.next = next_node.next;
                    cur_node.prev = this_node_pos;
                    next_node.next = next_node_pos;
                    arena[this_node_pos] = next_node;
                    arena[next_node_pos] = cur_node.clone();
                }
            }
        }
    }

    decrypt_once(&mut arena);

    fn print_result(arena: &Vec<Node>) {
        let mut temp = &arena[0];
        while temp.value != 0 {
            temp = &arena[temp.next];
        }

        let mut total = 0;
        for thou in 1..=3 {
            for _ in 0..(1000 % arena.len()) {
                temp = &arena[temp.next];
            }
            println!("{thou}000: {}", temp.value);
            total += temp.value;
        }
        println!("total: {total}");
    }

    print_result(&arena);

    for _ in 0..10 {
        decrypt_once(&mut arena_pt_2);
    }
    print_result(&arena_pt_2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
