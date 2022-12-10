fn update_sum(cycle_num: &mut i32, x: &mut i32, sum: &mut i32) {
    if *cycle_num == 20 || (*cycle_num - 20) % 40 == 0 {
        let signal_strength = *cycle_num * *x;
        *sum += signal_strength;
    }
}

fn draw_pixels(cycle_num: i32, x: i32) {
    let pixel_pos = (cycle_num - 1) % 40;
    if pixel_pos == x-1 || pixel_pos == x || pixel_pos == x+1 {
        print!("#");
    } else {
        print!(".");
    }
    if pixel_pos == 39 {
        println!("");
    }
}

fn main() {
    let mut cycle_num = 0;
    let mut x = 1;
    let mut sum = 0;
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }

        let mut part_num = 0;
        for part in line.split(' ') {
            if part_num == 0 {
                if part == "noop" {
                    cycle_num += 1;
                } else {
                    cycle_num += 1;
                }
                draw_pixels(cycle_num, x);
                update_sum(&mut cycle_num, &mut x, &mut sum);
                if part != "noop" {
                    cycle_num += 1;
                    draw_pixels(cycle_num, x);
                    update_sum(&mut cycle_num, &mut x, &mut sum);
                }
            } else if part_num == 1 {
                let delta = base::parse_int(part);
                x += delta;
            }
            part_num += 1;
        }
        
    }
    println!("Cycle: {}; X: {}; Sum: {}", cycle_num, x, sum);
}
