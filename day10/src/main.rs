fn update_sum(cycle_num: &mut i32, x: &mut i32, sum: &mut i32) {
    if *cycle_num == 20 || (*cycle_num - 20) % 40 == 0 {
        let signal_strength = *cycle_num * *x;
        println!("Cycle: {}, X: {}, Signal: {}", cycle_num, x, signal_strength);
        *sum += signal_strength;
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
                update_sum(&mut cycle_num, &mut x, &mut sum);
                if part != "noop" {
                    cycle_num += 1;
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
