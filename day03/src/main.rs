use std::env;
use std::io;
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
    return (num_bytes, line)
}

fn main() {
    let mut total = 0;
    let mut line_num = 0;
    let mut group_lines: [String; 3] = [
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];
    let part: u32 = env::args().nth(1)
            .unwrap_or("1".to_string())
            .parse()
            .expect("Failed to parse part arg");
    loop {
        let (num_bytes, line) = read_line();
        if num_bytes == 0 {
            break
        }

        
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
            break;
        }
        let value = get_priority(matched);
        println!("Bag {line} has matching item: {matched} with value {value}");
        total += value;
    }
    println!("Total: {total}");
}
