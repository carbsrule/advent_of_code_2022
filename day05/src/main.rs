use base;
use std::io;
use lazy_static::lazy_static;
use regex::Regex;

/**
 * Initialise stacks based on line length if necessary
 */
fn init_stacks(stacks: &mut Vec<String>, line: &String) {
    if stacks.len() == 0 {
        let num_stacks = ((line.len() + 1) as f64 / 4.0).round() as u32;
        println!("Num stacks: {num_stacks}");
        for _ in 0..num_stacks {
            stacks.push(String::new());
        }
    }
}

/**
 * Load line onto stacks
 *
 * Return false when no more stack lines to load
*/
fn load_stacks(stacks: &mut Vec<String>, line: String) -> bool {
    println!("Line: {line}");
    for i in 0..stacks.len() {
        let pos = (i * 4) + 1;
        let ch = line.chars().nth(pos).unwrap();
        if ch == '1' {
            // Stacked items have been read from top-bottom; want bottom-top
            for j in 0..stacks.len() {
                let mut reversed = String::new();
                while stacks[j].len() > 0 {
                    reversed.push(stacks[j].pop().unwrap());
                }
                stacks[j] = reversed;
            }
            return false;
        }
        if ch != ' ' {
            stacks[i].push(ch);
        }
    }
    return true;
}

fn do_move(stacks: &mut Vec<String>, line: String, part: u32) {
    lazy_static! {
        static ref RE: Regex = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    }
    for cap in RE.captures_iter(&line) {
        let count = base::parse_int(&cap[1]);
        let mut src = base::parse_int(&cap[2]) as usize;
        let mut dest = base::parse_int(&cap[3]) as usize;

        // Count from zero (vector indexes) instead of 1 (text)
        src -= 1;
        dest -= 1;

        if part == 2 {
            let mut crane_load = String::new();
            for _ in 0..count {
                let item = stacks[src].pop().unwrap();
                crane_load.push(item);
            }
            for _ in 0..count {
                let item = crane_load.pop().unwrap();
                stacks[dest].push(item);
            }
        } else {
            for _ in 0..count {
                let item = stacks[src].pop().unwrap();
                stacks[dest].push(item);
            }
        }
    }
}

fn main() {
    let part = base::get_part();
    let mut stacks: Vec<String> = vec![];
    let mut reading_stacks = true;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if num_bytes == 0 {
            break;
        }
        line = line.trim_matches(|c| c == '\r' || c == '\n').to_string();

        if line == "" {
            println!("Initial stacks: {:?}", stacks);
            continue;
        }

        if reading_stacks {
            init_stacks(&mut stacks, &line);
            reading_stacks = load_stacks(&mut stacks, line);
        } else {
            do_move(&mut stacks, line, part);
        }
    }
    println!("Final stacks: {:?}", stacks);

    let mut top_items = String::new();
    for i in 0..stacks.len() {
        top_items.push(stacks[i].chars().last().unwrap());
    }
    println!("Top items: {}", top_items);
}
