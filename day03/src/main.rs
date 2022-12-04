use std::env;
use std::io;
use std::process;
use substring::Substring;

fn get_priority(ch: char) -> u32 {
    if ch.is_uppercase() {
        return 27 + ch as u32 - 'A' as u32;
    } else {
        return 1 + ch as u32 - 'a' as u32;
    }
}

fn read_line() -> (usize, String) {
    let mut line = String::new();
    let num_bytes = io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    if num_bytes == 0 {
        line = "".to_string();
    } else {
        line = line.trim().to_string();
    }
    return (num_bytes, line);
}

fn get_shared_item(line: &String) -> char {
    let length = line.len();
    let half = length / 2;
    let comp1 = line.substring(0, half);
    let comp2 = line.substring(half, length);

    let mut pos = 0;
    let mut matched = ' ';
    let mut chars1 = comp1.chars();
    loop {
        if pos == half {
            break;
        }
        let match_char = chars1.nth(0).unwrap_or('?');
        if comp2.contains(match_char) {
            matched = match_char;
            break;
        }
        pos += 1;
    }
    if matched == ' ' {
        println!("*** No match found in bag {line}");
        process::exit(1);
    }
    return matched;
}

fn get_group_shared_item(group_lines: &[String; 3]) -> char {
    let first_line = &group_lines[0];
    let mut first_line_chars = first_line.chars();
    loop {
        let match_char = first_line_chars.nth(0).unwrap();
        if !group_lines[1].contains(match_char) {
            continue;
        }
        if !group_lines[2].contains(match_char) {
            continue;
        }
        return match_char;
    }
}

fn main() {
    let mut total = 0;
    let mut line_num = 0;
    let mut group_lines: [String; 3] = [
        String::new(),
        String::new(),
        String::new(),
    ];
    let mut num_bytes;
    let part: u32 = env::args().nth(1)
            .unwrap_or("1".to_string())
            .parse()
            .expect("Failed to parse part arg");
    loop {
        let mut group_line = 0;
        if part == 2 {
            group_line = line_num % 3;
        }
        (num_bytes, group_lines[group_line]) = read_line();
        // println!("{}", group_lines[group_line]);
        if num_bytes == 0 {
            break
        }

        if part == 2 {
            if group_line == 2 {
                println!("Bags: {}, {}, {}", group_lines[0], group_lines[1], group_lines[2]);
                let shared_item = get_group_shared_item(&group_lines);
                let value = get_priority(shared_item);
                println!("Group has shared item: {shared_item} with value {value}");
                total += value;
            }
        } else {
            let shared_item = get_shared_item(&group_lines[0]);
            let value = get_priority(shared_item);
            let line = &group_lines[0];
            println!("Bag {line} has shared item: {shared_item} with value {value}");
            total += value;
        }
        line_num += 1;
    }
    println!("Total: {total}");
}
