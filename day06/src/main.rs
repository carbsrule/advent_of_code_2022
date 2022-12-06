const BUFFER_LEN: usize = 14;

fn rearrange(chars: &mut [char; BUFFER_LEN], len: usize) {
    for i in 0..len - 1 {
        chars[i] = chars[i + 1];
    }
}

fn has_dup(chars: &[char; BUFFER_LEN], len: usize) -> bool {
    if chars[0] == ' ' {
        return true;
    }
    for j in 0..len {
        for k in j+1..len {
            if chars[j] == chars[k] {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let part = base::get_part();
    let check_len: usize = if part == 2 { 14 } else { 4 };
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }

        let mut i = 0;
        let mut chars: [char; 14] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
        for ch in line.chars() {
            rearrange(&mut chars, check_len);
            chars[check_len -1] = ch;
            // println!("Rearranged packet: {:?} at pos {}", chars, i);

            if !has_dup(&chars, check_len) {
                println!("Start packet: {:?}; packet at pos {}", chars, i+1);
                break;
            }
            i += 1;
        }
    }
}
