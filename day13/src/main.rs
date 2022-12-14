use std::fmt;

const DEBUG_OUTPUT: bool = false;

struct Item {
    is_list: bool,
    number: i32,
    list: List,
}
impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_list {
            write!(f, "{}", format!("{:?}", self.list))
        } else {
            write!(f, "{}", self.number)
        }
    }
}
impl Item {
    fn new_num(num: i32) -> Item {
        Item {
            is_list: false,
            number: num,
            list: List::new(),
        }
    }

    fn new_list(list: List) -> Item {
        Item {
            is_list: true,
            number: 0,
            list: list,
        }
    }
}

struct List {
    items: Vec<Item>,
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        out += "[";
        for i in 0..self.items.len() {
            if i > 0 {
                out += ",";
            }
            out += &format!("{:?}", self.items[i]);
        }
        out += "]";
        write!(f, "{}", out)
    }
}

impl List {
    fn new() -> List {
        List {
            items: vec![],
        }
    }

    fn add_str_num(&mut self, str: String) {
        if str != "" {
            let item_num = base::parse_int(&str);
            self.add_num(item_num);
        }
    }

    fn add_num(&mut self, num: i32) {
        let item = Item::new_num(num);
        self.items.push(item);
    }
}

fn read_list(chars: &Vec<char>, char_pos: &mut usize) -> List {
    let mut list= List::new();
    let mut found_start = false;
    let mut number = String::new();

    let len = chars.len();
    while *char_pos < len {
        let ch = chars[*char_pos];
        match ch {
            '[' => {
                if !found_start {
                    found_start = true;
                } else {
                    let child = read_list(chars, char_pos);
                    let child_item = Item::new_list(child);
                    list.items.push(child_item);
                }
            },
            ']' => {
                *char_pos += 1;
                list.add_str_num(number);
                return list;
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                number.push(ch);
            },
            ',' => {
                list.add_str_num(number);
                number = String::new();
            },
            _ => todo!(),
        }
        *char_pos += 1;
    }

    return list;
}

fn pad_spaces(depth: u8) -> String {
    let mut spaces = String::new();
    for _ in 0..depth {
        spaces.push_str("  ");
    }
    return spaces;
}

fn compare_numbers(item_a: &Item, item_b: &Item, depth: u8) -> i8 {
    let spaces = pad_spaces(depth);
    if DEBUG_OUTPUT {
        println!("{}- Compare {:?} vs {:?}", spaces, item_a.number, item_b.number);
    }
    if item_a.number < item_b.number {
        return -1;
    } else if item_a.number == item_b.number {
        return 0;
    }
    return 1;
}

fn compare_lists(a: &List, b: &List, depth: u8) -> i8 {
    let spaces = pad_spaces(depth);
    if DEBUG_OUTPUT {
        println!("{}- Compare {:?} vs {:?}", spaces, a, b);
    }
    for i in 0..a.items.len() {
        // b has ran out of items
        if i >= b.items.len() {
            if DEBUG_OUTPUT {
                println!("{}- out of RHS items", spaces);
                println!("{}- LHS at {}: {:?}", spaces, i, a.items[i]);
            }
            return 1;
        }

        let item_a = &a.items[i];
        let item_b = &b.items[i];

        let res: i8;
        if !item_a.is_list && !item_b.is_list {
            res = compare_numbers(item_a, item_b, depth + 1);
        } else if item_a.is_list && item_b.is_list {
            res = compare_lists(&item_a.list, &item_b.list, depth + 1);
        } else {
            // Mixed types
            if item_a.is_list {
                let mut b_list = List::new();
                b_list.add_num(item_b.number);
                res = compare_lists(&item_a.list, &b_list, depth + 1);
            } else {
                let mut a_list = List::new();
                a_list.add_num(item_a.number);
                res = compare_lists(&a_list, &item_b.list, depth + 1);
            }
        }
        if res != 0 {
            if DEBUG_OUTPUT {
                println!("{}- Comparison finished with: {}", spaces, res);
            }
            return res;
        }
    }
    if b.items.len() > a.items.len() {
        return -1;
    }
    return 0;
}

fn to_char_vec(str: &String) -> Vec<char> {
    let mut chars: Vec<char> = vec![];
    for char in str.chars() {
        chars.push(char);
    }
    return chars;
}

fn sort(list: &mut Vec<List>) {
    loop {
        let mut sorted = false;
        for i in 0..list.len() - 1 {
            let res = compare_lists(&list[i], &list[i+1], 0);
            if res != 1 {
                continue;
            }
            sorted = true;
            list.swap(i, i+1);
        }
        if !sorted {
            break;
        }
    }
}

fn main() {
    let mut side = 'L';
    let mut list: List;
    let mut left_idx: usize = 0;
    let mut right_idx: usize;
    let mut sum_correct_indices = 0;
    let mut pair_num = 0;

    let mut all_entries: Vec<List> = vec![];
    let div1 = "[[2]]".to_string();
    let div2 = "[[6]]".to_string();
    let mut divider_packet = to_char_vec(&div1);
    all_entries.push(read_list(&divider_packet, &mut 0));
    divider_packet = to_char_vec(&div2);
    all_entries.push(read_list(&divider_packet, &mut 0));
    
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0  {
            break
        }

        if line == "" {
            continue;
        }

        // read left or right line (alternating)
        let chars = to_char_vec(&line);

        // Ignore comments
        if chars[0] == '#' {
            continue;
        }

        let mut char_pos: usize = 0;
        list = read_list(&chars, &mut char_pos);
        all_entries.push(list);
        if side == 'L' {
            left_idx = all_entries.len() - 1;
            side = 'R';
        } else {
            right_idx = all_entries.len() - 1;
            side = 'L';

            // do comparison
            let left = &all_entries[left_idx];
            let right = &all_entries[right_idx];
            pair_num += 1;
            let comp_result = compare_lists(left, right, 0);

            let comp_info = if comp_result == 1 {
                "out of order"
            } else {
                "in order"
            };
            // println!("{:?} vs {:?}: {}", left, right, comp_info);
            // println!("");

            if comp_result != 1 {
                sum_correct_indices += pair_num;
            }
        }
    }
    println!("Total in correct order: {}", sum_correct_indices);

    sort(&mut all_entries);

    let mut decoder = 0;
    for i in 0..all_entries.len() {
        let list_repr = format!("{:?}", all_entries[i]);
        println!("Sorted entry {}: {}", i, list_repr);
        if list_repr == div1 {
            decoder = i + 1;
        } else if list_repr == div2 {
            decoder *= i + 1;
        }
    }
    println!("Decoder key: {}", decoder);
}
