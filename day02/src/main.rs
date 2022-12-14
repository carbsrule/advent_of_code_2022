use std::env;
use std::io;
use std::process;
use substring::Substring;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

fn calc_score(letter: &str) -> u32 {
    match letter {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS,
        "X" => ROCK,
        "Y" => PAPER,
        "Z" => SCISSORS,
        &_ => {
            println!("Unknown: {letter}");
            process::exit(1)
        },
    }
}

fn calc_response(opp: u32, my: &str) -> u32 {
    // X: lose, Y: draw, Z: win
    if my == "Y" {
        return opp;
    }
    if opp == ROCK {
        match my {
            "X" => SCISSORS,
            "Z" => PAPER,
            &_ => process::exit(2),
        }
    } else if opp == PAPER {
        match my {
            "X" => ROCK,
            "Z" => SCISSORS,
            &_ => process::exit(3),
        }
    } else { // opp == SCISSORS
        match my {
            "X" => PAPER,
            "Z" => ROCK,
            &_ => process::exit(4),
        }
    }
}

fn calc_result(opp: u32, my: u32) -> [u32; 2] {
    if opp == my {
        return [3, 3];
    }

    if opp == ROCK && my == SCISSORS {
        return [6, 0];
    } else if opp == PAPER && my == ROCK {
        return [6, 0];
    } else if opp == SCISSORS && my == PAPER {
        return [6, 0];
    } else {
        let reverse = calc_result(my, opp);
        return [reverse[1], reverse[0]];
    }
}

fn main() {
    let part = env::args().nth(1).unwrap_or("1".to_string());
    let mut my_total = 0;
    loop {
        let mut line = String::new();
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        if num_bytes == 0 {
            break
        }

        let opp_letter = line.substring(0,1);
        let my_letter = line.substring(2,3);
        println!("{opp_letter} vs {my_letter}");

        let opp_score = calc_score(opp_letter);
        let my_score;
        if part == "2" {
            my_score = calc_response(opp_score, my_letter);
        } else {
            my_score = calc_score(my_letter);
        }

        let mut result = calc_result(opp_score, my_score);
        result[0] += opp_score;
        result[1] += my_score;
        
        println!("Score: {} vs {}", result[0], result[1]);
        my_total += result[1];
    }
    println!("My total: {my_total}");
}
