use std::fmt;
use std::cmp;

const DEBUG_OUTPUT: bool = false;

// #[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

fn update_tail_positions(tail_pos: &Point, tail_positions: &mut Vec<Point>) {
    for i in 0..tail_positions.len() {
        let check_pos = &tail_positions[i];
        if check_pos.x == tail_pos.x && check_pos.y == tail_pos.y {
            return
        }
    }

    // No match, add new position
    let new_pos = Point{
        x: tail_pos.x,
        y: tail_pos.y,
    };
    tail_positions.push(new_pos);
}

fn rope_move(dir: &str, dist: i32, pos: &mut Point, tail_pos: &mut Point, tail_positions: &mut Vec<Point>) {
    let mut delta = Point{
        x: 0,
        y: 0,
    };
    match dir {
        "R" => delta.x = 1,
        "L" => delta.x = -1,
        "U" => delta.y = 1,
        "D" => delta.y = -1,
        _ => println!("Fail: {}", dir),
    }
    if DEBUG_OUTPUT {
        println!("{} {} ->", dir, dist);
    }
    
    for _ in 0..dist {
        pos.x += delta.x;
        pos.y += delta.y;
        
        let tail_delta_x = pos.x - tail_pos.x;
        let tail_delta_y = pos.y - tail_pos.y;
        let tail_delta = cmp::max(tail_delta_x.abs(), tail_delta_y.abs());
        if tail_delta > 1 && tail_delta_x > 0 {
            if DEBUG_OUTPUT {
                println!("Move tail right: {}", tail_delta);
            }
            tail_pos.x += 1;
        } else if tail_delta > 1 && tail_delta_x < 0 {
            if DEBUG_OUTPUT {
                println!("Move tail left: {}", tail_delta);
            }
            tail_pos.x -= 1;
        }

        if tail_delta > 1 && tail_delta_y > 0 {
            if DEBUG_OUTPUT {
                println!("Move tail up: {}", tail_delta);
            }
            tail_pos.y += 1;
            
        } else if tail_delta > 1 && tail_delta_y < 0 {
            if DEBUG_OUTPUT {
                println!("Move tail down: {}", tail_delta);
            }
            tail_pos.y -= 1;
        }
        if DEBUG_OUTPUT {
            println!(" {:?} (tail {:?})", (pos.x, pos.y), (tail_pos.x, tail_pos.y));
        }
        update_tail_positions(&tail_pos, tail_positions);
    }
}

fn main() {
    let mut pos = Point{
        x: 0,
        y: 0,
    };
    let mut tail_pos = Point{
        x: 0,
        y: 0,
    };
    let mut tail_positions: Vec<Point> = vec![];
    update_tail_positions(&tail_pos, &mut tail_positions);
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }
        
        let mut dir = "";
        let mut dist = 0;
        
        let parts = line.split(' ');
        let mut i = 0;
        for part in parts {
            if i == 0 {
                dir = part;
            }
            if i == 1 {
                dist = base::parse_int(part);
            }
            i += 1;
        }
        rope_move(dir, dist, &mut pos, &mut tail_pos, &mut tail_positions);
    }
    println!("Tail positions: {}", tail_positions.len());
}
