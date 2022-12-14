use std::fmt;

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
    println!("{}- Compare {:?} vs {:?}", spaces, item_a.number, item_b.number);
    if item_a.number < item_b.number {
        return -1;
    } else if item_a.number == item_b.number {
        return 0;
    }
    return 1;
}

fn compare_lists(a: &List, b: &List, depth: u8) -> i8 {
    let spaces = pad_spaces(depth);
    println!("{}- Compare {:?} vs {:?}", spaces, a, b);
    for i in 0..a.items.len() {
        // b has ran out of items
        if i >= b.items.len() {
            println!("{}- out of RHS items", spaces);
            println!("{}- LHS at {}: {:?}", spaces, i, a.items[i]);
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
            println!("{}- Comparison finished with: {}", spaces, res);
            return res;
        }
    }
    if b.items.len() > a.items.len() {
        return -1;
    }
    return 0;
}

fn main() {
    let mut side = 'L';
    let mut left = List::new();
    let mut right;
    let mut sum_correct_indices = 0;
    let mut pair_num = 0;
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0  {
            break
        }

        if line == "" {
            continue;
        }

        // read left or right line (alternating)
        let mut chars: Vec<char> = vec![];
        for char in line.chars() {
            chars.push(char);
        }

        // Ignore comments
        if chars[0] == '#' {
            continue;
        }

        let mut char_pos: usize = 0;
        if side == 'L' {
            left = read_list(&chars, &mut char_pos);
            side = 'R';
        } else {
            right = read_list(&chars, &mut char_pos);
            side = 'L';

            // do comparison
            pair_num += 1;
            let comp_result = compare_lists(&left, &right, 0);

            let comp_info = if comp_result == 1 {
                "out of order"
            } else {
                "in order"
            };
            println!("{:?} vs {:?}: {}", left, right, comp_info);
            println!("");

            if comp_result != 1 {
                sum_correct_indices += pair_num;
            }
        }
    }
    println!("Total in correct order: {}", sum_correct_indices);
}
