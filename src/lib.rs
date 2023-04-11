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

fn hex_chunk_to_base64(x: &[u8]) -> String {
    assert!(x.len() > 0);
    let mut values: Vec<u8> = vec![];
    // First value is guaranteed, only depends on the first byte
    values.push(x[0] >> 2);
    // Second value depends on the first byte, and optionally on the second byte
    let mask = if x.len() > 1 { x[1] >> 4 } else { 0 };
    values.push((0b11 & x[0]) << 4 | mask);

    if x.len() > 1 {
        // Third value depends on the second byte, and optionally on the third byte
        let mask = if x.len() > 2 { x[2] >> 6 } else { 0 };
        values.push((0b1111 & x[1]) << 2 | mask);
    }
    if x.len() > 2 {
        // Fourth values depends only on the third byte
        values.push(0b111111 & x[2]);
    }

    let a_string: String = values.into_iter().map(base64_as_ascii).collect();
    format!("{:=<width$}", a_string, width = 4)
}

pub fn hex_to_base64(input: String) -> String {
    let decoded_hex_bytes: Vec<u8> = (0..input.len())
        .step_by(2)
        .map(|i| match u8::from_str_radix(&input[i..i + 2], 16) {
            Ok(res) => res,
            Err(_) => panic!("err"),
        })
        .collect();

    decoded_hex_bytes
        .chunks(3)
        .fold("".to_string(), |acc, x| acc + &hex_chunk_to_base64(x))
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
