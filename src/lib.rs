
pub fn hex_to_base64(input: String) -> String {
    let ascii_base64 = hex_bytes_to_base64(ascii_hex_to_bytes(input))
        .iter()
        .map(base64_as_ascii)
        .collect::<String>();

    // Pad the ascii output if needed
    let last_chunk = ascii_base64.len() % 4;
    if last_chunk > 0 {
        // Should be ok as the + operator will steal the buffer from left side
        ascii_base64 + &str::repeat("=", 4 - last_chunk)
    } else {
        ascii_base64
    }
}

fn ascii_hex_to_bytes(input: String) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| match u8::from_str_radix(&input[i..i + 2], 16) {
            Ok(res) => res,
            Err(e) => panic!("Invalid hex pair value. Error: {:?}", e),
        })
        .collect()
}

fn base64_as_ascii(b: &u8) -> char {
    let res = match b {
        0..=25 => 'A' as u8 + b,
        26..=51 => 'a' as u8 + b - 26,
        52..=61 => '0' as u8 + b - 52,
        62 => '+' as u8,
        63 => '/' as u8,
        _ => unreachable!(),
    };
    res as char
}

pub fn hex_bytes_to_base64(input: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> =
        Vec::with_capacity(4 * (input.len() / 3 + usize::from(input.len() % 3 != 0)));

    let exact_chunks = input.chunks_exact(3);
    let remainder = exact_chunks.remainder();
    for chunk in exact_chunks {
        res.push(chunk[0] >> 2);

        res.push(0b00111111 & (chunk[0] << 4 | chunk[1] >> 4));

        res.push(0b00111111 & (chunk[1] << 2 | chunk[2] >> 6));

        res.push(0b00111111 & chunk[2]);
    }

    match remainder {
        [x] => {
            res.push(x >> 2);
            res.push(0b00110000 & x << 4);
        }
        [x, y] => {
            res.push(x >> 2);
            res.push(0b00111111 & (x << 4 | y >> 4));
            res.push(0b00111100 & y << 2);
        }
        _ => {}
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hex_to_base64(String::from("")), "");
        assert_eq!(hex_to_base64(String::from("4d616e")), "TWFu");
        assert_eq!(hex_to_base64(String::from("4d61")), "TWE=");
        assert_eq!(hex_to_base64(String::from("4d")), "TQ==");
        assert_eq!(hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}
