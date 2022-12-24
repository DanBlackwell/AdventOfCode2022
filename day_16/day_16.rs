use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct ValveChamber {
    flow_rate: usize,
    connected_valves: Vec<String>
}

#[derive(Debug)]
struct Node {
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
                    flow_rate: flow_rate,
                    connected_valves: valve_strings
                });
            }
        }
    }

    let mut nodes = HashMap::new();

    for (start_name, start_valve) in valves.iter().filter(|(n,v)| v.flow_rate > 0 || *n == "AA") {
        let mut node = Node {
            flow_rate: start_valve.flow_rate,
            shortest_path_to: HashMap::new()
        };

        for (end_name, _end_valve) in valves.iter().filter(|(k,v)| v.flow_rate > 0 && *k != start_name) {
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
    queue.push((
        State { 
            cur_node: "AA".to_string(), 
            flow_rate: 0, 
            score: 0,
            time: 0,
            open_valves: Vec::new(),
            open_times: Vec::new(),
        },
        State { 
            cur_node: "AA".to_string(), 
            flow_rate: 0, 
            score: 0,
            time: 0,
            open_valves: Vec::new(),
            open_times: Vec::new(),
        }
    ));

    let max_flow_rate = nodes.iter().fold(0, |acc,(_,v)| acc + v.flow_rate);
    println!("max_flow_rate: {max_flow_rate}");

    let all_useful_nodes = nodes.iter()
        .filter(|(_,v)| v.flow_rate > 0)
        .map(|(k,_)| k.clone()).collect::<HashSet<String>>();

    const END_TIME: usize = 26;

    let mut highest = 0;
    while queue.len() > 0 {
        let (my_state, ele_state) = queue.pop().unwrap();

        if my_state.time >= END_TIME && ele_state.time >= END_TIME {
            let my_score = my_state.score - (my_state.time - END_TIME) * my_state.flow_rate;
            let ele_score = ele_state.score - (ele_state.time - END_TIME) * ele_state.flow_rate;
            if my_score + ele_score > highest {
                highest = my_score + ele_score;
                println!("me:  cur_score: {}, flow_rate: {}, time: {}, new highest: {highest}, queue: {}", 
                    my_state.score, my_state.flow_rate, my_state.time, queue.len());
                println!("{:?}\n{:?}", my_state.open_valves, my_state.open_times);
                println!("ele: cur_score: {}, flow_rate: {}, time: {}, new highest: {highest}, queue: {}", 
                    ele_state.score, ele_state.flow_rate, ele_state.time, queue.len());
                println!("{:?}\n{:?}", ele_state.open_valves, ele_state.open_times);
            }
            continue;
        }

        let my_turn = my_state.time < ele_state.time;
        let node = nodes.get(if my_turn { &my_state.cur_node } else { &ele_state.cur_node }).unwrap();

        // A* like pruning
        const OPTIMISTIC_AVG: usize = 80;
        let my_max = my_state.score + (END_TIME - my_state.time) * OPTIMISTIC_AVG;
        let ele_max = ele_state.score + (END_TIME - ele_state.time) * OPTIMISTIC_AVG;
        if my_max + ele_max < highest {
            continue;
        }

        // if ele_state.flow_rate + my_state.flow_rate == max_flow_rate {
        //     my_state.score += (END_TIME - my_state.time) * my_state.flow_rate;
        //     my_state.time = END_TIME;
        //     ele_state.score += (END_TIME - ele_state.time) * ele_state.flow_rate;
        //     ele_state.time = END_TIME;

        //     if my_state.score + ele_state.score > highest {
        //         highest = my_state.score + ele_state.score;
        //         println!("me:  new highest: {highest}, state: {:?}, queue: {}", 
        //             my_state, queue.len());
        //         println!("{:?}\n{:?}", my_state.open_valves, my_state.open_times);
        //         println!("ele:  new highest: {highest}, state: {:?}, queue: {}", 
        //             ele_state, queue.len());
        //         println!("{:?}\n{:?}", ele_state.open_valves, ele_state.open_times);
        //     }
        //     continue;
        // }

        let mut open = my_state.open_valves.clone().into_iter().collect::<HashSet<String>>();
        for valve in ele_state.open_valves.clone() { open.insert(valve); }
        let mut uncovered = all_useful_nodes.difference(&open).collect::<Vec<&String>>();
        uncovered.sort_by(|a,b| 
            nodes.get(*a).unwrap().flow_rate.partial_cmp(
                &nodes.get(*b).unwrap().flow_rate
            ).unwrap()
        );

        let mut too_long_pushed = false;
        for name in uncovered {
            let mut next = if my_turn { my_state.clone() } else { ele_state.clone() };
            let travel_time = node.shortest_path_to.get(name).unwrap();
            if next.time + travel_time + 1 >= END_TIME {
                next.score += (END_TIME - next.time) * next.flow_rate;
                next.time = END_TIME;
                if too_long_pushed {
                    continue;
                } else {
                    too_long_pushed = true;
                }
            } else {
                next.time += travel_time + 1;
                next.score += (travel_time + 1) * next.flow_rate;
                next.cur_node = name.clone();
                next.open_valves.push(name.clone());
                next.open_times.push(next.time - 1);
                next.flow_rate += nodes.get(name).unwrap().flow_rate;
            }
            let next_tuple = if my_turn { (next, ele_state.clone()) } else { (my_state.clone(), next) };
            queue.push(next_tuple);
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
