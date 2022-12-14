use substring::Substring;

struct Monkey {
    items: Vec<i64>,
    operator: String,
    operand: i64,
    div_test: i64,
    true_to: usize,
    false_to: usize,
    total_inspections: u64,
}

fn parse_monkey_data(line: &mut String, monkeys: &mut Vec<Monkey>, mut monkey_idx: usize) -> usize {
    if line.substring(0, 7) == "Monkey " {
        let new_idx = base::parse_int(line.substring(7, line.len() - 1)) as usize;
        monkeys.push(Monkey {
            items: vec![],
            operator: "".to_string(),
            operand: 0,
            div_test: 1,
            true_to: 0,
            false_to: 0,
            total_inspections: 0,
        });
        return new_idx;
    } else if line.substring(0, 15) == "Starting items:" {
        for item in line.substring(15, line.len()).split(',') {
            let item = base::parse_int(item.trim()) as i64;
            monkeys[monkey_idx].items.push(item);
        }
    } else if line.substring(0, 10) == "Operation:" {
        let mut parts = line.substring(11, line.len()).split(' ');        
        monkeys[monkey_idx].operator = parts.nth(3).unwrap().to_string();
        let operand = parts.next().unwrap();
        if operand != "old" {
            monkeys[monkey_idx].operand = base::parse_int(operand) as i64;
        }
        
    } else if line.substring(0, 5) == "Test:" {
        monkeys[monkey_idx].div_test = base::parse_int(line.substring(19, line.len())) as i64;
        // Test: divisible by 17
    } else if line.substring(0, 8) == "If true:" {
        let parts = line.substring(11, line.len()).split(' ');
        monkeys[monkey_idx].true_to = base::parse_int(parts.last().unwrap()) as usize;
    } else if line.substring(0, 9) == "If false:" {
        let parts = line.substring(11, line.len()).split(' ');
        monkeys[monkey_idx].false_to = base::parse_int(parts.last().unwrap()) as usize;
    } else {
        assert!(line == "");
        monkey_idx += 1;
    }
    return monkey_idx;
}

fn prepare_throws(monkeys: &mut Vec<Monkey>, idx: usize, part: u32) -> Vec<(usize, i64)> {
    let monkey = &mut monkeys[idx];
    let mut throws = vec![];
    for i in 0..monkey.items.len() {
        let item = monkey.items[i];
        monkey.total_inspections += 1;

        let mut operand = monkey.operand;
        if operand == 0 {
            operand = item;
        }
        let mut worry_level = item;
        if monkey.operator == "+" {
            worry_level += operand;
        } else {
            assert!(monkey.operator == "*");
            worry_level *= operand;
        }
        if part == 1 {
            worry_level /= 3;
        }
        let throw_to = if worry_level % monkey.div_test == 0 { monkey.true_to } else { monkey.false_to };
        throws.push((throw_to, worry_level));
    }
    return throws;
}

fn main() {
    let part = base::get_part();
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_idx: usize = 0;
    loop {
        let (num_bytes, mut line) = base::read_line();
        if num_bytes == 0 {
            break;
        }

        line = line.trim().to_string();
        monkey_idx = parse_monkey_data(&mut line, &mut monkeys, monkey_idx);
    }

    // Handle large numbers using modulo as product of modulos for all operations
    let mut mass_divisor = 1;
    for i in 0..monkeys.len() {
        let monkey = &monkeys[i];
        println!(
            "{}: {:?} {} {} /{} {}-{}", i,
            monkey.items, monkey.operator, monkey.operand, monkey.div_test,
            monkey.true_to, monkey.false_to,
        );
        mass_divisor *= monkey.div_test;
    }

    let rounds = if part == 2 { 10000 } else { 20 };
    for _round in 1..=rounds {
        for i in 0..monkeys.len() {
            let throws = prepare_throws(&mut monkeys, i, part);
            for throw in throws {
                let target = throw.0;
                let worry_value = throw.1 % mass_divisor;
                monkeys[target].items.push(worry_value);
            }
            monkeys[i].items = vec![];
        }
        // println!("After round {_round}, the monkeys are holding items with these worry levels:");
        // for i in 0..monkeys.len() {
        //     println!("Monkey {i}, inspected: {}, items: {:?}", monkeys[i].total_inspections, monkeys[i].items);
        // }
    }

    let mut most_active: [u64; 2] = [0, 0];
    for i in 0..monkeys.len() {
        let monkey_total = monkeys[i].total_inspections;
        println!("Monkey {i}: inspected items {} times", monkey_total);
        if monkey_total > most_active[0] {
            most_active[1] = most_active[0];
            most_active[0] = monkey_total;
        } else if monkey_total > most_active[1] {
            most_active[1] = monkey_total;
        }
    }
    println!("Most active: {:?}", most_active);
    println!("Monkey business: {}", most_active[0] * most_active[1]);
}
