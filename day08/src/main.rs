fn is_visible(col_num: usize, row_num: usize, lines: &Vec<String>) -> bool {
    let tree_row = &lines[row_num];
    let tree = tree_row.chars().nth(col_num).unwrap();
    let tree_height = base::char_parse_int(&tree);

    if tree_height == 0 {
        // Cannot have adjacent trees lower than 0
        return false;
    }

    let mut visible = true;
    for col in 0..tree_row.len() {
        if col == col_num {
            if visible == true {
                return true;
            }
            visible = true;
            continue;
        }
        let compare_tree = tree_row.chars().nth(col).unwrap();
        let compare_height = base::char_parse_int(&compare_tree);
        if compare_height >= tree_height {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for row in 0..lines.len() {
        if row == row_num {
            if visible == true {
                return true;
            }
            visible = true;
            continue;
        }
        let compare_tree = lines[row].chars().nth(col_num).unwrap();
        let compare_height = base::char_parse_int(&compare_tree);
        if compare_height >= tree_height {
            visible = false;
        }
    }
    if visible {
        return true;
    }
    
    return false;
}

fn calc_score(col_num: usize, row_num: usize, lines: &Vec<String>) -> i32 {
    let tree_row = &lines[row_num];
    let tree = tree_row.chars().nth(col_num).unwrap();
    let tree_height = base::char_parse_int(&tree);

    let mut left = 0;
    for col in (0..col_num).rev() {
        let compare_tree = tree_row.chars().nth(col).unwrap();
        let compare_height = base::char_parse_int(&compare_tree);
        left += 1;
        if compare_height >= tree_height {
            break;
        }
    }

    let mut right = 0;
    for col in col_num+1..tree_row.len() {
        let compare_tree = tree_row.chars().nth(col).unwrap();
        let compare_height = base::char_parse_int(&compare_tree);
        right += 1;
        if compare_height >= tree_height {
            break;
        }
    }
    
    let mut up = 0;
    for row in (0..row_num).rev() {
        let compare_tree = lines[row].chars().nth(col_num).unwrap();
        let compare_height = base::char_parse_int(&compare_tree);
        up += 1;
        if compare_height >= tree_height {
            break;
        }
    }

    let mut down = 0;
    for row in row_num+1..lines.len() {
        let compare_tree = lines[row].chars().nth(col_num).unwrap();
        let compare_height = base::char_parse_int(&compare_tree);
        down += 1;
        if compare_height >= tree_height {
            break;
        }
    }

    let total = left * right * up * down;
    // println!("[{col_num}, {row_num}]: l{left}, r{right}, u{up}, d{down} = {total}");
    return total;
}

fn main() {
    let mut lines = vec![];
    let mut cols = 0;
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }
        if cols == 0 {
            cols = line.len();
        }
        lines.push(line);
    }
    let rows = lines.len();

    let mut visible_trees = 0;
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if is_visible(col, row, &lines) {
                visible_trees += 1;
            }
        }
    }

    // edge trees are all visible
    visible_trees += (cols * 2) + (rows * 2) - 4;
    
    println!("Visible trees: {}", visible_trees);

    let mut max_scenic_score = 0;
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            let scenic_score = calc_score(col, row, &lines);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("Max scenic score: {}", max_scenic_score);
}
