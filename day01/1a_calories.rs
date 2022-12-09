use std::io;

fn main() {
    let mut max = 0;
    let mut elf_with_most = 0;
    let mut total = 0;
    let mut calories: u32;
    let mut elf_num = 1;

    println!("Elf {elf_num}");
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        if num_bytes == 0 {
            break
        }

        line = line.trim().to_string();
        if line == "" {
            total = 0;
            elf_num += 1;
            println!("\nElf {elf_num}");
            continue;
        }

        calories = line.parse().expect("Failed to parse number");
        total += calories;
        if total > max {
            max = total;
            elf_with_most = elf_num;
        }
        println!("Calories: {calories} (total {total}, max {max})");
    }
    println!("Elf {elf_with_most}/{elf_num} has most: {max} calories");
}
