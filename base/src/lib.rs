use std::env;
use std::io;

pub fn get_part() -> u32 {
    let part: u32 = env::args().nth(1)
        .unwrap_or("1".to_string())
        .parse()
        .expect("Failed to parse part arg");
    if part != 2 {
        return 1;
    }
    return part;
}

pub fn read_line() -> (usize, String) {
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

pub fn parse_int(num: &str) -> i32 {
    return num.parse().expect("Failed to parse number");
}

pub fn char_parse_int(ch: &char) -> i32 {
    let mut num = String::new();
    num.push(*ch);
    return parse_int(&num);
}
