use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct ValveChamber {
    name: String,
    flow_rate: usize,
    connected_valves: Vec<String>
}

#[derive(Debug)]
struct Node {
    name: String,
    flow_rate: usize,
    shortest_path_to: HashMap<String, usize>
}

#[derive(Debug,Clone)]
struct State {
    cur_node: String,
    flow_rate: usize,
    score: usize,
    time: usize,
    open_valves: Vec<String>,
    open_times: Vec<usize>
}

fn main() {
    let mut valves: HashMap<String, ValveChamber> = HashMap::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let words = line.split(" ").collect::<Vec<&str>>();
                // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
                let valve_name = words[1].to_string();
                let flow_rate_str = words[4].split("=").nth(1).unwrap().split(";").nth(0).unwrap();
                let flow_rate = flow_rate_str.parse::<usize>().unwrap();
                let valve_list = words[9..].join("");
                let valve_strings = valve_list.split(",").map(|a| a.to_string()).collect::<Vec<String>>();

                valves.insert(valve_name.clone(), ValveChamber {
                    name: valve_name,
                    flow_rate: flow_rate,
                    connected_valves: valve_strings
                });
            }
        }
    }

    let mut nodes = HashMap::new();

    for (start_name, start_valve) in valves.iter().filter(|(n,v)| v.flow_rate > 0 || *n == "AA") {
        let mut node = Node {
            name: start_name.clone().to_string(),
            flow_rate: start_valve.flow_rate,
            shortest_path_to: HashMap::new()
        };

        for (end_name, end_valve) in valves.iter().filter(|(k,v)| v.flow_rate > 0 && *k != start_name) {
            let mut queue = Vec::from([(start_name.clone(), 0, HashSet::new())]);
            let mut shortest = 1_000_000;

            while queue.len() > 0 {
                let (name, depth, visited) = queue.pop().unwrap();

                let valve = valves.get(&name).unwrap();
                for neighbour in &valve.connected_valves {
                    if *neighbour == *end_name {
                        if depth + 1 < shortest {
                            shortest = depth + 1;
                            node.shortest_path_to.insert(end_name.clone(), shortest);
                        }
                        continue;
                    } else if visited.contains(neighbour) {
                        continue;
                    }

                    let mut new_visited = visited.clone();
                    new_visited.insert(name.clone());
                    queue.push((neighbour.clone(), depth + 1, new_visited));
                }
            }
        }

        nodes.insert(start_name.clone(), node);
    }

    let mut queue = Vec::new();
    queue.push(State { 
        cur_node: "AA".to_string(), 
        flow_rate: 0, 
        score: 0,
        time: 0,
        open_valves: Vec::new(),
        open_times: Vec::new(),
    });

    let max_flow_rate = nodes.iter().fold(0, |acc,(k,v)| acc + v.flow_rate);
    println!("max_flow_rate: {max_flow_rate}");

    let all_useful_nodes = nodes.iter()
        .filter(|(k,v)| v.flow_rate > 0)
        .map(|(k,v)| k.clone()).collect::<HashSet<String>>();

    const END_TIME: usize = 30;

    let mut highest = 0;
    let mut latest_time = 0;
    while queue.len() > 0 {
        let mut cur_state = queue.pop().unwrap();
        let node = nodes.get(&cur_state.cur_node).unwrap();

        if cur_state.time + 1 >= END_TIME {
            let score = cur_state.score - (cur_state.time - END_TIME) * cur_state.flow_rate;
            if score > highest {
                highest = score;
                println!("cur_score: {}, flow_rate: {}, time: {}, new highest: {highest}, queue: {}", 
                    cur_state.score, cur_state.flow_rate, cur_state.time, queue.len());
                println!("{:?}\n{:?}", cur_state.open_valves, cur_state.open_times);
            }
            continue;
        }

        if node.flow_rate > 0 {
            cur_state.open_valves.push(cur_state.cur_node.clone());
            cur_state.open_times.push(cur_state.time);
            cur_state.time += 1;
            cur_state.score += cur_state.flow_rate;
            cur_state.flow_rate += node.flow_rate;
        }

        // even if we magically opened all valves we couldn't do better than what we found so far
        if cur_state.score + (END_TIME - cur_state.time) * max_flow_rate < highest {
            continue;
        }

        if cur_state.flow_rate == max_flow_rate {
            let remaining_time = END_TIME - cur_state.time;
            cur_state.time = END_TIME;
            cur_state.score += remaining_time * cur_state.flow_rate;
        }

        // if node.flow_rate > 0 && !cur_state.open_valves.contains(&node.name) {
        //     let mut next = cur_state.clone();
        //     next.time += 1;
        //     next.score += cur_state.flow_rate;
        //     next.flow_rate += node.flow_rate;
        //     next.open_valves.push(node.name.clone());
        //     next.open_times.push(next.time);

        //     queue.push(next);
        // }

        let open = cur_state.open_valves.clone().into_iter().collect::<HashSet<String>>();
        // println!("covered nodes: {:?}", all_nodes.union(&open.clone()));
        let mut uncovered = all_useful_nodes.difference(&open).collect::<Vec<&String>>();
        uncovered.sort_by(|a,b| 
            nodes.get(*a).unwrap().flow_rate.partial_cmp(
                &nodes.get(*b).unwrap().flow_rate
            ).unwrap()
        );

        // println!("uncovered: {:?}", uncovered);
        for name in uncovered {
            if *name == cur_state.cur_node { continue; }
            let mut next = cur_state.clone();
            let travel_time = node.shortest_path_to.get(name).unwrap();
            next.time += travel_time;
            next.score += travel_time * cur_state.flow_rate;
            next.cur_node = name.clone();
            queue.push(next);
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
