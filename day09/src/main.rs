use std::cmp;
use std::env;
use std::fmt;
use std::io;
use std::io::Write;
use std::process;

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

fn update_tail_history(knots: &Vec<Point>, tail_history: &mut Vec<Point>) {
    let tail_pos = &knots[knots.len() - 1];
    for i in 0..tail_history.len() {
        let check_pos = &tail_history[i];
        if check_pos.x == tail_pos.x && check_pos.y == tail_pos.y {
            return
        }
    }

    // No match, add new position
    let new_pos = Point{
        x: tail_pos.x,
        y: tail_pos.y,
    };
    tail_history.push(new_pos);
}

fn move_knot(knots: &mut Vec<Point>, knot_idx: usize, delta: &Point) {
    if knot_idx >= knots.len() {
        return;
    }

    knots[knot_idx].x += delta.x;
    knots[knot_idx].y += delta.y;

    let knot = &knots[knot_idx];

    let next_idx = knot_idx + 1;
    if next_idx >= knots.len() {
        return
    }

    let next_knot = &knots[next_idx];
    let next_delta_x = knot.x - next_knot.x;
    let next_delta_y = knot.y - next_knot.y;
    let next_delta = cmp::max(next_delta_x.abs(), next_delta_y.abs());
    let mut next_move = Point{
        x: 0,
        y: 0,
    };
    if next_delta > 1 && next_delta_x > 0 {
        if DEBUG_OUTPUT {
            println!("Move {next_idx} right: {}", next_delta);
        }
        next_move.x += 1;
    } else if next_delta > 1 && next_delta_x < 0 {
        if DEBUG_OUTPUT {
            println!("Move {next_idx} left: {}", next_delta);
        }
        next_move.x -= 1;
    }

    if next_delta > 1 && next_delta_y > 0 {
        if DEBUG_OUTPUT {
            println!("Move {next_idx} up: {}", next_delta);
        }
        next_move.y += 1;
    } else if next_delta > 1 && next_delta_y < 0 {
        if DEBUG_OUTPUT {
            println!("Move {next_idx} down: {}", next_delta);
        }
        next_move.y -= 1;
    }

    if DEBUG_OUTPUT {
        println!(" {:?} ({next_idx} move {:?})", (knot.x, knot.y), (next_move.x, next_move.y));
    }
    if next_move.x != 0 || next_move.y != 0 {
        move_knot(knots, next_idx, &next_move);
    }
}

fn rope_move(dir: &str, dist: i32, knots: &mut Vec<Point>, tail_history: &mut Vec<Point>) {
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
        move_knot(knots, 0, &delta);
        update_tail_history(&knots, tail_history);
    }
}

fn main() {
    let num_knots = env::args().nth(1)
        .unwrap()
        .parse()
        .expect("Failed to parse arg: number of knots");
    if num_knots < 2 {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        let _ = handle.write_all(b"Must have at least 2 knots\n");
        process::exit(2);
    }

    let mut knots: Vec<Point> = vec![];
    for _ in 0..num_knots {
        knots.push(Point{
            x: 0,
            y: 0,
        });
    }

    let mut tail_history: Vec<Point> = vec![];
    update_tail_history(&knots, &mut tail_history);
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
        rope_move(dir, dist, &mut knots, &mut tail_history);
    }
    println!("Tail positions: {}", tail_history.len());
}
