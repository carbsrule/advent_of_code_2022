const START_PACKET_LEN: usize = 4;

fn rearrange(chars: &mut [char; START_PACKET_LEN]) {
    for i in 0..START_PACKET_LEN - 1 {
        chars[i] = chars[i + 1];
    }
}

fn has_dup(chars: &[char; START_PACKET_LEN]) -> bool {
    if chars[0] == ' ' {
        return true;
    }
    for j in 0..START_PACKET_LEN {
        for k in j+1..START_PACKET_LEN {
            if chars[j] == chars[k] {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    loop {
        let (num_bytes, line) = base::read_line();
        if num_bytes == 0 {
            break;
        }

        let mut i = 0;
        let mut chars: [char; 4] = [' ', ' ', ' ', ' '];
        for ch in line.chars() {
            rearrange(&mut chars);
            chars[START_PACKET_LEN-1] = ch;
            // println!("Rearranged packet: {:?} at pos {}", chars, i);

            if !has_dup(&chars) {
                println!("Start packet: {:?}; message at pos {}", chars, i+1);
                break;
            }
            i += 1;
        }
    }
}
