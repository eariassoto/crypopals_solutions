fn base64_as_ascii(b: u8) -> char {
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

pub fn hex_to_base64(input: String) -> String {
    let decoded_hex_bytes: Vec<u8> = (0..input.len())
        .step_by(2)
        .map(|i| match u8::from_str_radix(&input[i..i + 2], 16) {
            Ok(res) => res,
            Err(_) => panic!("err"),
        })
        .collect();

    let res_capacity = decoded_hex_bytes.len() / 3 + usize::from(decoded_hex_bytes.len() % 3 != 0);
    let res_capacity = res_capacity * 4;
    let mut res = String::with_capacity(res_capacity);

    let exact_chunks = decoded_hex_bytes.chunks_exact(3);
    let remainder = exact_chunks.remainder();
    for chunk in exact_chunks {
        res.push(base64_as_ascii(chunk[0] >> 2));

        res.push(base64_as_ascii(0b00111111 & (chunk[0] << 4 | chunk[1] >> 4)));

        res.push(base64_as_ascii(0b00111111 & (chunk[1] << 2 | chunk[2] >> 6)));

        res.push(base64_as_ascii(0b00111111 & chunk[2]));
    }

    match remainder {
        [x] => {
            // todo use similar masks
            res.push(base64_as_ascii(x >> 2));
            res.push(base64_as_ascii((0b11 & x) << 4));
            res.push_str("==");
        }
        [x, y] => {
            res.push(base64_as_ascii(x >> 2));
            let mask = y >> 4;
            res.push(base64_as_ascii((0b11 & x) << 4 | mask));
            res.push(base64_as_ascii((0b1111 & y) << 2));
            res.push('=');
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
        assert_eq!(hex_to_base64(String::from("4d616e")), "TWFu");
        assert_eq!(hex_to_base64(String::from("4d61")), "TWE=");
        assert_eq!(hex_to_base64(String::from("4d")), "TQ==");
        assert_eq!(hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}
