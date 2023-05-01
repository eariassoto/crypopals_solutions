const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let input = input.as_ref();
    let mut res = String::with_capacity(4 * (input.len() / 3 + usize::from(input.len() % 3 != 0)));

    let exact_chunks = input.chunks_exact(3);
    let remainder = exact_chunks.remainder();
    for chunk in exact_chunks {
        let base64_byte = chunk[0] >> 2;
        res.push(BASE64_CHARS[base64_byte as usize]);

        let base64_byte = 0b00111111 & (chunk[0] << 4 | chunk[1] >> 4);
        res.push(BASE64_CHARS[base64_byte as usize]);

        let base64_byte = 0b00111111 & (chunk[1] << 2 | chunk[2] >> 6);
        res.push(BASE64_CHARS[base64_byte as usize]);

        let base64_byte = 0b00111111 & chunk[2];
        res.push(BASE64_CHARS[base64_byte as usize]);
    }

    match remainder {
        [x] => {
            let base64_byte = x >> 2;
            res.push(BASE64_CHARS[base64_byte as usize]);

            let base64_byte = 0b00111111 & x << 4;
            res.push(BASE64_CHARS[base64_byte as usize]);
        }
        [x, y] => {
            let base64_byte = x >> 2;
            res.push(BASE64_CHARS[base64_byte as usize]);

            let base64_byte = 0b00111111 & (x << 4 | y >> 4);
            res.push(BASE64_CHARS[base64_byte as usize]);

            let base64_byte = 0b00111100 & y << 2;
            res.push(BASE64_CHARS[base64_byte as usize]);
        }
        _ => {}
    }

    let remainder = res.len() % 4;
    if remainder > 0 {
        // Should be ok as the + operator will steal the buffer from left side
        res + &str::repeat("=", 4 - remainder)
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        assert_eq!(encode(vec![]), String::from(""));
        assert_eq!(encode(vec![0x4d, 0x61, 0x6e]), String::from("TWFu"));
        assert_eq!(encode(vec![0x4d, 0x61]), String::from("TWE="));
        assert_eq!(encode(vec![0x4d]), String::from("TQ=="));
    }
}
