use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(PartialEq, Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Debug)]
struct Robot {
    output: Resource,
    inputs: Vec<(Resource, usize)>,
}

type Blueprint = Vec<Robot>;

fn main() {
    let mut blueprints: Vec<Blueprint> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }

                let mut new_blueprint: Blueprint = Vec::new();
                
                let robots = line.split(":").nth(1).unwrap();
                for robot_str in robots.split(".") {
                    if robot_str.len() < 2 { continue; }

                    let mut split = robot_str.split(" ");

                    fn parse_resource(raw_str: Option<&str>) -> Resource {
                        // println!("parsing resource: {:?}", raw_str);
                        return match raw_str {
                            None => panic!(),
                            Some(output) =>  match output {
                                "ore" => Resource::Ore,
                                "clay" => Resource::Clay,
                                "obsidian" => Resource::Obsidian,
                                "geode" => Resource::Geode,
                                &_ => panic!(),
                            }
                        };
                    }

                    let out = parse_resource(split.nth(2));

                    let in_quant_1 = split.nth(2).unwrap().parse::<usize>().unwrap();
                    let in1 = parse_resource(split.nth(0));

                    let robot: Robot;
                    if split.clone().count() > 1 {
                        let in_quant_2 = split.nth(1).unwrap().parse::<usize>().unwrap();
                        let in2 = parse_resource(split.nth(0));

                        robot = Robot {
                            output: out,
                            inputs: Vec::from([(in1, in_quant_1), (in2, in_quant_2)]),
                        };
                    } else {
                        robot = Robot {
                            output: out,
                            inputs: Vec::from([(in1, in_quant_1)]),
                        };
                    }

                    new_blueprint.push(robot);
                }

                blueprints.push(new_blueprint);
            }
        }
    }

    let mut quality_total = 0;
    let mut part_2_total = 1;

    // Blueprint 1: Each ore robot costs 4 ore. 
    // Each clay robot costs 2 ore. 
    // Each obsidian robot costs 3 ore and 14 clay. 
    // Each geode robot costs 2 ore and 7 obsidian.
    for (index, blueprint) in blueprints.into_iter().enumerate() {
        let get_robot = |resource: Resource| -> &Robot {
            let robots: Vec<&Robot> = blueprint
                .iter()
                .filter(|r| r.output == resource)
                .collect();
            if robots.len() != 1 { panic!(""); }
            return robots[0];
        };

        let mut max_rates = [0; 4];
        for i in 0..4 {
            let max_robots_from_inventory = |input: Resource| -> usize {
                blueprint.iter().fold(0, |max, robot| {
                    let required = robot.inputs.iter().fold(0, |max, r| {
                        let (i, quantity) = r; 
                        if *i == input { 
                            return std::cmp::max(max, *quantity);
                        } else {
                            return max;
                        }
                    });

                    return std::cmp::max(max, required);
                })
            };

            max_rates[i] = match i {
                0 => max_robots_from_inventory(Resource::Ore),
                1 => max_robots_from_inventory(Resource::Clay),
                2 => max_robots_from_inventory(Resource::Obsidian),
                3 => max_robots_from_inventory(Resource::Geode),
                _ => panic!(),
            };
        }

        let initial_state = FactoryState {
            time: 1,

            ore_robot_plan: get_robot(Resource::Ore),
            clay_robot_plan: get_robot(Resource::Clay),
            obsidian_robot_plan: get_robot(Resource::Obsidian),
            geode_robot_plan: get_robot(Resource::Geode),

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,

            max_ore_robots_per_min: max_rates[0],
            max_clay_robots_per_min: max_rates[1],
            max_obsidian_robots_per_min: max_rates[2],
        };

        let max = dfs(&blueprint, initial_state.clone(), 1, 24);
        quality_total += (index + 1) * max;

        if index < 3 {
            let max = dfs(&blueprint, initial_state, 1, 32);
            part_2_total *= max;
        }
        println!("Processed {index}");
    }

    println!("Part 1 Quality total: {quality_total}");
    println!("Part 2 total: {part_2_total}");
}

#[derive(Debug, Clone)]
struct FactoryState<'a> {
    time: usize,

    ore_robot_plan: &'a Robot,
    clay_robot_plan: &'a Robot,
    obsidian_robot_plan: &'a Robot,
    geode_robot_plan: &'a Robot,

    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,

    ore_count: usize,
    clay_count: usize,
    obsidian_count: usize,
    geode_count: usize,

    max_ore_robots_per_min: usize,
    max_clay_robots_per_min: usize,
    max_obsidian_robots_per_min: usize,
}

fn can_create_robot_instance(robot: &Robot, state: &FactoryState) -> bool {
    for (resource, quantity) in &robot.inputs {
        match resource {
            Resource::Ore => if state.ore_count < *quantity { return false; },
            Resource::Clay => if state.clay_count < *quantity { return false; },
            Resource::Obsidian => if state.obsidian_count < *quantity { return false; },
            Resource::Geode => if state.geode_count < *quantity { return false; },
        };
    }

    return true;
}

fn create_robot_instance(robot: &Robot, state: &mut FactoryState) {
    for (resource, quantity) in &robot.inputs {
        match resource {
            Resource::Ore => state.ore_count -= *quantity,
            Resource::Clay => state.clay_count -= *quantity,
            Resource::Obsidian => state.obsidian_count -= *quantity,
            Resource::Geode => state.geode_count -= *quantity,
        };
    }

    match robot.output {
        Resource::Ore => state.ore_robots += 1,
        Resource::Clay => state.clay_robots += 1,
        Resource::Obsidian => state.obsidian_robots += 1,
        Resource::Geode => state.geode_robots += 1,
    }
}

fn do_nothing(state: &mut FactoryState) {
    state.time += 1;
    state.ore_count += state.ore_robots;
    state.clay_count += state.clay_robots;
    state.obsidian_count += state.obsidian_robots;
    state.geode_count += state.geode_robots;
}

fn dfs(blueprint: &Blueprint, state: FactoryState, depth: usize, max_time: usize) -> usize {
    if state.time > max_time { panic!(); }
    let mut cur_state = state.clone();

    let mut max_geodes = 0;
    for i in 0..4 {

        let robot_plan: &Robot;
         match i {
            0 => { robot_plan = state.geode_robot_plan; },
            1 => {
                // Don't build more robots than we could use the output of
                if cur_state.obsidian_count > state.max_obsidian_robots_per_min * (max_time - cur_state.time) { continue; }
                robot_plan = state.obsidian_robot_plan;
            },
            2 => {
                if cur_state.clay_count > state.max_clay_robots_per_min * (max_time - cur_state.time) { continue; }
                robot_plan = state.clay_robot_plan;
            },
            3 => {
                if cur_state.ore_count > state.max_ore_robots_per_min * (max_time - cur_state.time) { continue; }
                robot_plan = state.ore_robot_plan;
            },
            _ => panic!()
        };

        // Don't try to wait for resources to build a robot if we aren't mining that resource
        let mut are_mining_resources = true;
        for (resource, _) in robot_plan.inputs.iter() {
            match resource {
                Resource::Ore => continue,
                Resource::Clay => if cur_state.clay_robots == 0 { are_mining_resources = false; },
                Resource::Obsidian => if cur_state.obsidian_robots == 0 { are_mining_resources = false; },
                Resource::Geode => if cur_state.geode_robots == 0 { are_mining_resources = false; },
            }
        }
        if !are_mining_resources { continue; }

        let mut loop_state = cur_state.clone();

        let mut out_of_time = loop_state.time >= max_time;
        while !out_of_time && !can_create_robot_instance(robot_plan, &loop_state) {
            do_nothing(&mut loop_state);
            // If we can't create a new one in time, that's fine as we calc the run to end below
            if loop_state.time >= max_time { out_of_time = true; }
        }
        if out_of_time { continue; }

        do_nothing(&mut loop_state);

        create_robot_instance(robot_plan, &mut loop_state);
        let outcome = dfs(blueprint, loop_state.clone(), depth + 1, max_time);
        if outcome > max_geodes { max_geodes = outcome; }
    }

    while cur_state.time <= max_time {
        do_nothing(&mut cur_state);
    }
    max_geodes = std::cmp::max(cur_state.geode_count, max_geodes); 

    return max_geodes;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
