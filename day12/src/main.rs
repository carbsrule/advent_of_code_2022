use std::collections::VecDeque;
use std::fmt;

#[derive(PartialEq)]
#[derive(Clone)]
struct Point(usize, usize);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}


struct Map {
    start: Point,
    end: Point,
    grid: Vec<Vec<u8>>,
    paths: VecDeque<Vec<Point>>,
}

fn get_letter_height(ch: char) -> u8 {
    match ch {
        'S' => 0,
        'E' => 25,
        _ => ch as u8 - 'a' as u8,
    }
}

fn get_point_height(map: &Map, point: &Point) -> u8 {
    return map.grid[point.0][point.1];
}

fn get_point_letter(map: &Map, point: &Point) -> char {
    let height = get_point_height(map, point);
    return ('a' as u8 + height) as char;
}

fn read_map() -> Map {
    let mut map = Map {
        start: Point(0, 0),
        end: Point(0, 0),
        grid: vec![],
        paths: VecDeque::new(),
    };

    let mut col: usize;
    let mut row: usize = 0;
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            return map;
        }

        if map.grid.len() <= row {
            let new_row = vec![];
            map.grid.push(new_row);
        }

        col = 0;
        for ch in line.chars() {
            if ch == 'S' {
                map.start = Point(row, col);
                map.paths.push_back(vec![Point(row, col)]);
            } else if ch == 'E' {
                map.end = Point(row, col);
            }
            map.grid[row].push(get_letter_height(ch));
            col += 1;
        }
        row += 1;
    }
}

fn show_map(map: &Map) {
    for row in 0..map.grid.len() {
        print!("Row {row}: ");
        for col in 0..map.grid[row].len() {
            print!("{},", map.grid[row][col]);
        }
        println!("");
    }
}

fn move_possible(src: &Point, dest: &Point, map: &Map) -> bool {
    for i in 0..map.paths.len() {
        if map.paths[i].contains(dest) {
            return false;
        }
    }

    // Max. height increase of 1; can jump down from any height
    if get_point_height(map, dest) > get_point_height(map, src) + 1 {
        return false;
    }
    return true;
}

fn extend_path(map: &Map, path: &Vec<Point>) -> Vec<Vec<Point>> {
    let end = path.last().unwrap();
    let previous: &Point = if path.len() > 2 {
         &path[path.len() - 2]
    } else {
        &Point(100_000_000, 100_000_000)
    };

    let mut directional_moves: Vec<Point> = vec![];
    if end.0 > 0 {
        directional_moves.push(Point(end.0 - 1, end.1)); // up
    }
    if end.0 < map.grid.len() - 1 {
        directional_moves.push(Point(end.0 + 1, end.1)); // down
    }
    if end.1 > 0 {
        directional_moves.push(Point(end.0, end.1 - 1)); // left
    }
    if end.1 < map.grid[0].len() - 1 {
        directional_moves.push(Point(end.0, end.1 + 1)); // right
    }

    let mut possible_moves: Vec<Point> = vec![];
    for possible_move in directional_moves {
        if possible_move == *previous {
            continue;
        }
        if move_possible(end, &possible_move, map) {
            possible_moves.push(possible_move);
        }
    }

    let mut new_paths: Vec<Vec<Point>> = vec![];
    let num_poss = possible_moves.len();
    if num_poss == 0 {
        return new_paths;
    }

    for i in 0..num_poss {
        let new_move = &possible_moves[i];
        let mut new_path: Vec<Point> = vec![];
        for j in 0..path.len() {
            new_path.push(path[j].clone());
        }
        new_path.push(new_move.clone());
        new_paths.push(new_path);
    }
    return new_paths;
}

fn find_path(map: &mut Map) -> Vec<Point> {
    loop {
        if map.paths.len() == 0 {
            return vec![];
        }
        
        let path = &map.paths[0];
        let new_paths = extend_path(&map, path);
        map.paths.remove(0);
        for i in 0..new_paths.len() {
            let tail = &new_paths[i][new_paths[i].len() - 1];
            if *tail == map.end {
                return new_paths[i].clone();
            }
            map.paths.push_back(new_paths[i].clone());
        }
    }
}

fn find_alt_start_points(map: &Map) -> Vec<Point> {
    let mut start_points = vec![];
    for i in 0..map.grid.len() {
        for j in 0..map.grid[i].len() {
            if map.grid[i][j] == 0 {
                let start_point = Point(i, j);
                if map.start != start_point {
                    start_points.push(start_point);
                }
            }
        }
    }
    return start_points;
}

fn main() {
    let mut map = read_map();

    let mut start_points = vec![map.start.clone()];
    let mut alt_starts = find_alt_start_points(&map);
    start_points.append(&mut alt_starts);

    let mut shortest_path = 100_000_000;
    for i in 0..start_points.len() {
        map.paths = VecDeque::new();
        map.paths.push_back(vec![start_points[i].clone()]);

        let start = &start_points[i];
        map.start = start.clone();
        let path = find_path(&mut map);
        if path.len() == 0 {
            println!("No path from {:?} to finish", &start);
            continue;
        }

        let path_len = path.len() - 1;
        println!("Path from {:?} to finish has {} steps", &start, path_len);
        if path_len - 1 < shortest_path {
            shortest_path = path_len;
        }
    }
    println!("Shortest path: {shortest_path}");
}
