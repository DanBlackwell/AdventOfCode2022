use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    Equal
}

#[derive(Debug, Clone)]
struct Node {
    value: Option<f64>,
    child_1: Option<String>,
    child_2: Option<String>,
    op: Option<Op>,
}

fn main() {
    let mut monkeys: HashMap<String, Node> = HashMap::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                let mut split = line.split(":");
                let monkey_name = split.nth(0).unwrap().to_string();

                let node;

                let mut val_split = split.nth(0).unwrap().split(" ");
                if val_split.clone().count() == 2 {
                    let constant = val_split.nth(1).unwrap().parse::<f64>().unwrap();

                    node = Node {
                        value: Some(constant),
                        child_1: None,
                        child_2: None,
                        op: None,
                    };

                } else {
                    let monk_1 = val_split.nth(1).unwrap().to_string();
                    let operation = match val_split.nth(0).unwrap() {
                        "+" => if monkey_name.clone() == "root" { Op::Equal } else { Op::Plus },
                        "-" => Op::Minus,
                        "*" => Op::Mult,
                        "/" => Op::Div,
                        _   => panic!()
                    };
                    let monk_2 = val_split.nth(0).unwrap().to_string();

                    node = Node {
                        value: None,
                        child_1: Some(monk_1),
                        child_2: Some(monk_2),
                        op: Some(operation)
                    }
                }

                monkeys.insert(monkey_name, node);
            }
        }
    }

    fn get_value(monkey_name: String, monkeys: &HashMap<String, Node>, part_1: bool) -> (f64, Option<(f64, f64)>) {
        let monkey = monkeys.get(&monkey_name).unwrap();
        if let Some(constant) = monkey.value {
            return (constant, None);
        }

        let child_1_val = get_value(monkey.child_1.clone().unwrap(), monkeys, part_1).0;
        let child_2_val = get_value(monkey.child_2.clone().unwrap(), monkeys, part_1).0;
        let calced = match monkey.op.clone().unwrap() {
            Op::Plus => child_1_val + child_2_val,
            Op::Minus => child_1_val - child_2_val,
            Op::Mult => child_1_val * child_2_val,
            Op::Div => child_1_val / child_2_val,
            Op::Equal => {
                if part_1 {
                    println!("part_1: {}", child_1_val + child_2_val);
                }
                if child_1_val == child_2_val { 1.0 } else { 0.0 }
            },
        };

        if monkey.op.clone().unwrap() == Op::Equal {
            return (calced, Some((child_1_val, child_2_val)));
        }

        return (calced, None);
    }

    get_value("root".to_string(), &monkeys, true);

    let mut human = monkeys.get(&"humn".to_string()).unwrap().clone();
    let mut bounds = (-1_000_000_000_000_000.0, 1_000_000_000_000_000.0);

    human.value = Some(bounds.0);
    monkeys.insert("humn".to_string(), human.clone());

    let expected = get_value("root".to_string(), &monkeys, false).1.unwrap().1;

    let mut lower_val = get_value("root".to_string(), &monkeys, false).1.unwrap().0;

    human.value = Some(bounds.1);
    monkeys.insert("humn".to_string(), human.clone());
    let mut upper_val = get_value("root".to_string(), &monkeys, false).1.unwrap().0;

    // Binary search
    loop {
        if upper_val < lower_val {
            let temp = upper_val;
            upper_val = lower_val;
            lower_val = temp;

            bounds = (bounds.1, bounds.0);
        }

        let middle = (bounds.0 + bounds.1) / 2.0;
        human.value = Some(middle);
        monkeys.insert("humn".to_string(), human.clone());
        let middle_val = get_value("root".to_string(), &monkeys, false).1.unwrap().0;

        if expected < lower_val || expected > upper_val { panic!(); }
        if expected >= lower_val && expected < middle_val {
            upper_val = middle_val;
            bounds.1 = middle;
        } else if expected <= upper_val && expected > middle_val {
            lower_val = middle_val;
            bounds.0 = middle;
        } else {
            println!("part 2 Found the expected value with human input: {middle}");
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
