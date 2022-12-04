use base;

fn fully_contained(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    if start1 <= start2 && end1 >= end2 {
        return true;
    }
    if start2 <= start1 && end2 >= end1 {
        return true;
    }
    return false;
}

fn overlap(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    if fully_contained(start1, end1, start2, end2) {
        return true;
    }
    if start1 <= start2 && end1 >= start2 {
        return true;
    }
    if start1 <= end2 && end1 >= end2 {
        return true;
    }
    return false;
}

fn main() {
    let part = base::get_part();

    let mut total = 0;
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }

        let ranges: Vec<&str> = line.split(",").collect();
        let range1: Vec<&str> = ranges[0].split("-").collect();
        let range2: Vec<&str> = ranges[1].split("-").collect();
        let start1 = base::parse_int(range1[0]);
        let end1 = base::parse_int(range1[1]);
        let start2 = base::parse_int(range2[0]);
        let end2 = base::parse_int(range2[1]);
        if part == 2 {
            if overlap(start1, end1, start2, end2) {
                println!("Overlap: {}", line);
                total += 1;
            }
        } else {
            if fully_contained(start1, end1, start2, end2) {
                println!("Fully contained: {}", line);
                total += 1;
            }
        }
    }
    println!("Total: {total}");
}
