use std::io;

static mut ELF1: u32 = 0;
static mut ELF1_CALORIES: u32 = 0;
static mut ELF2: u32 = 0;
static mut ELF2_CALORIES: u32 = 0;
static mut ELF3: u32 = 0;
static mut ELF3_CALORIES: u32 = 0;

fn sum(total: u32, elf_num: u32) {
    unsafe {
        if total > ELF1_CALORIES {
            ELF3 = ELF2;
            ELF3_CALORIES = ELF2_CALORIES;
            ELF2 = ELF1;
            ELF2_CALORIES = ELF1_CALORIES;
            ELF1 = elf_num;
            ELF1_CALORIES = total;
        } else if total > ELF2_CALORIES {
            ELF3 = ELF2;
            ELF3_CALORIES = ELF2_CALORIES;
            ELF2 = elf_num;
            ELF2_CALORIES = total;
        } else if total > ELF3_CALORIES {
            ELF3 = elf_num;
            ELF3_CALORIES = total;
        }
    }
}

fn main() {
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
            sum(total, elf_num);
            break
        }

        line = line.trim().to_string();
        if line == "" {
            sum(total, elf_num);
            total = 0;
            elf_num += 1;
            println!("\nElf {elf_num}");
            continue;
        }

        calories = line.parse().expect("Failed to parse number");
        total += calories;
        println!("Calories: {calories} (total {total})");
    }
    println!("Top 3 elves:");

    unsafe {
        println!("Elf {ELF1} with {ELF1_CALORIES} calories");
        println!("Elf {ELF2} with {ELF2_CALORIES} calories");
        println!("Elf {ELF3} with {ELF3_CALORIES} calories");
        let top3_cals = ELF1_CALORIES + ELF2_CALORIES + ELF3_CALORIES;
        println!("Altogether: {top3_cals} calories");
    }
}
