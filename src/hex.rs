const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn decode<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let input = input.as_ref();
    let mut res: Vec<u8> = (0..input.len() - 1)
        .step_by(2)
        .map(|i| match &input[i..i + 2] {
            [high_nibble @ b'0'..=b'9', low_nibble @ b'0'..=b'9'] => {
                (high_nibble - b'0') << 4 | (low_nibble - b'0')
            }
            [high_nibble @ b'a'..=b'f', low_nibble @ b'0'..=b'9']
            | [high_nibble @ b'A'..=b'F', low_nibble @ b'0'..=b'9'] => {
                (high_nibble.to_ascii_lowercase() - b'a' + 10) << 4 | (low_nibble - b'0')
            }
            [high_nibble @ b'0'..=b'9', low_nibble @ b'a'..=b'f']
            | [high_nibble @ b'0'..=b'9', low_nibble @ b'A'..=b'F'] => {
                (high_nibble - b'0') << 4 | (low_nibble.to_ascii_lowercase() - b'a' + 10)
            }
            [high_nibble @ b'a'..=b'f', low_nibble @ b'a'..=b'f']
            | [high_nibble @ b'A'..=b'F', low_nibble @ b'A'..=b'F'] => {
                (high_nibble.to_ascii_lowercase() - b'a' + 10) << 4
                    | (low_nibble.to_ascii_lowercase() - b'a' + 10)
            }
            _ => panic!("Invalid hex pair value."),
        })
        .collect();
    if input.len() % 2 == 1 {
        match &input.last().unwrap() {
            b @ b'0'..=b'9' => res.push(*b - b'0'),
            b @ b'a'..=b'f' | b @ b'A'..=b'F' => {
                res.push((b.to_ascii_lowercase() - b'a' + 10) << 4)
            }
            _ => panic!("Invalid hex pair value."),
        }
    }
    res
}

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let input = input.as_ref();
    let mut res = String::with_capacity(input.len());
    for byte in input {
        res.push(HEX_CHARS[(byte >> 4) as usize]);
        res.push(HEX_CHARS[(byte & 0x0F) as usize]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test() {
        assert_eq!(decode(String::from("")), vec![]);
        assert_eq!(decode(String::from("00")), vec![0x00]);
        assert_eq!(decode(String::from("12")), vec![0x12]);
        assert_eq!(
            decode(String::from("deadbeef")),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }
}
