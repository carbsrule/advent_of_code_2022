use std::collections::HashMap;

use substring::Substring;

struct Status {
    currdir: String,
    dirs: HashMap<String, u64>,
}

fn init_curr_dir(status: &mut Status) {
    if !status.dirs.contains_key(&status.currdir) {
        let key = status.currdir.clone();
        status.dirs.insert(key, 0);
    }
}

fn up_dir(dir: &str) -> &str {
    let slash_pos = dir.rfind('/');
    if slash_pos != None {
        let end = slash_pos.unwrap();
        let new_dir = if end == 0 { "/" } else { dir.substring(0, end) };
        return new_dir;
    }
    return dir;
}

fn do_cmd(command: &str, mut status: &mut Status) {
    let comm = command.to_string();
    if comm.substring(0, 3) == "cd " {
        let dir = comm.substring(3, comm.len()).to_string();
        print!("$ cd: '{dir}'");
        if status.currdir == "" {
            status.currdir = dir;
        } else {
            if dir == ".." {
                status.currdir = up_dir(&status.currdir).to_string();
            } else {
                if status.currdir != "/" {
                    status.currdir.push('/');
                }
                status.currdir.push_str(dir.as_str());
            }
        }
        init_curr_dir(&mut status);
        println!(" -> {}", status.currdir);
    }
}

fn handle_output(line: &str, status: &mut Status) {
    let line = line.to_string();

    if line.substring(0, 4) == "dir " {
        return;
    }
    let split: Vec<&str> = line.split(' ').collect();
    let size: u64 = split[0].parse().expect("Failed to parse number");

    let mut dir = &status.currdir;
    let mut new_dir;
    loop {
        match status.dirs.get(dir) {
            Some(dir_size) => {
                let new_size = dir_size + size;
                status.dirs.insert(dir.to_string(), new_size);
            },
            None => break,
        }
        if dir == "/" {
            break;
        }
        new_dir = up_dir(&dir).to_string();
        dir = &new_dir;
    }
}

fn main() {
    let dir_map: HashMap<String, u64> = HashMap::new();
    let mut status = Status {
        currdir: "".to_string(),
        dirs: dir_map,
    };
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }

        if line.substring(0, 2) == "$ " {
            let command = line.substring(2, line.len());
            do_cmd(&command, &mut status);
        } else {
            println!("Handling output: {}", line);
            handle_output(&line, &mut status);
        }
    }

    println!("{:?}", status.dirs);

    let size_limit = 100000;
    let mut total_size = 0;
    for (_, size) in &status.dirs {
        if size <= &size_limit {
            total_size += size;
        }
    }
    println!("Total size (dirs < {}): {}", size_limit, total_size);    

    let total_disk_space = 70000000;
    let mut space_used = 0;
    match status.dirs.get("/") {
        Some(dir_size) => {
            space_used = *dir_size;
        },
        None => return
    }
    println!("Space use: {}/{}", space_used, total_disk_space);

    let required_space = 30000000;
    let space_to_free = space_used - (total_disk_space - required_space);
    println!("Space to free: {}", space_to_free);

    let mut min_deletable = space_used;
    for (_, size) in &status.dirs {
        if size >= &space_to_free && size < &min_deletable {
            min_deletable = *size;
        }
    }
    println!("Can free: {}", min_deletable);
}
