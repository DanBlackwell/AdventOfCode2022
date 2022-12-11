fn main() {
    const PART_2: bool = true;
    let worry_reduction = if PART_2 {1} else {3};
    let rounds = if PART_2 {10_000} else {20};
    let modulo = 7 * 19 * 5 * 11 * 17 * 13 * 2 * 3;

    let mut monkey_items: Vec<Vec<u64>> = Vec::new();
    let mut monkey_func: Vec<Box<dyn Fn(&mut Vec<Vec<u64>>, usize)>> = Vec::new();
    let mut inspection_count: Vec<usize> = Vec::new();

    monkey_items.push(Vec::from([57, 58]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item * 19) / worry_reduction) % modulo;
           items[if updated % 7 == 0 { 2 } else { 3 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));

    monkey_items.push(Vec::from([66, 52, 59, 79, 94, 73]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item + 1) / worry_reduction) % modulo;
           items[if updated % 19 == 0 { 4 } else { 6 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));

    monkey_items.push(Vec::from([80]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item + 6) / worry_reduction) % modulo;
           items[if updated % 5 == 0 { 7 } else { 5 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));
    
    monkey_items.push(Vec::from([82, 81, 68, 66, 71, 83, 75, 97]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item + 5) / worry_reduction) % modulo;
           items[if updated % 11 == 0 { 5 } else { 2 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));
    
    monkey_items.push(Vec::from([55, 52, 67, 70, 69, 94, 90]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item * item) / worry_reduction) % modulo;
           items[if updated % 17 == 0 { 0 } else { 3 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));
    
    monkey_items.push(Vec::from([69, 85, 89, 91]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item + 7) / worry_reduction) % modulo;
           items[if updated % 13 == 0 { 1 } else { 7 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));
    
    monkey_items.push(Vec::from([75, 53, 73, 52, 75]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item * 7) / worry_reduction) % modulo;
           items[if updated % 2 == 0 { 0 } else { 4 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));
    
    monkey_items.push(Vec::from([94, 60, 79]));
    inspection_count.push(0);
    monkey_func.push(Box::new(|items: &mut Vec<Vec<u64>>, cur_monk: usize| {
       if let Some(item) = items[cur_monk].first() {
           let updated = ((item + 2) / worry_reduction) % modulo;
           items[if updated % 3 == 0 { 1 } else { 6 }].push(updated);
           items[cur_monk].drain(0..1);
       }
    }));

    for _round in 0..rounds {
        for (index, func) in monkey_func.iter().enumerate() {
            let to_inspect_count = monkey_items[index].len();
            inspection_count[index] += to_inspect_count;
            for _ in 0..to_inspect_count {
                func(&mut monkey_items, index);
            }
        }
    }
    
    inspection_count.sort();
    let result: usize = inspection_count.iter().rev().take(2).product();
    println!("{:?}, top 2: {}", inspection_count, result);
}
