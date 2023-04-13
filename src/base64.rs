
pub fn hex_to_base64(input: Vec<u8>) -> Vec<u8> {
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
    use crate::pretty_print::{bytes_base64_to_ascii, ascii_hex_to_bytes};

    use super::*;

    fn test_from_str(input: String, expected: String) {
        assert_eq!(
            bytes_base64_to_ascii(
                hex_to_base64(ascii_hex_to_bytes(input)),
                /*padding=*/ true
            ),
            expected
        );
    }

    #[test]
    fn it_works() {
        test_from_str(String::from("4d616e"), String::from("TWFu"));
        test_from_str(String::from("4d61"), String::from("TWE="));
        test_from_str(String::from("4d"), String::from("TQ=="));
        test_from_str(
            String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        );
    }
}
