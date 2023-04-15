const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn decode(input: String) -> Vec<u8> {
    // TODO: handle errors properly
    if input.len() % 2 == 1 {
        panic!("Invalid input.")
    }

    (0..input.len())
        .step_by(2)
        .map(|i| match u8::from_str_radix(&input[i..i + 2], 16) {
            Ok(res) => res,
            Err(e) => panic!("Invalid hex pair value. Error: {:?}", e),
        })
        .collect()
}

pub fn encode(input: Vec<u8>) -> String {
    let mut res = String::with_capacity(input.len());
    for byte in input {
        res.push(HEX_CHARS[(byte >> 4) as usize]);
        res.push(HEX_CHARS[(byte & 0x0F) as usize]);
    }
    res
}

pub fn fixed_xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    if a.len() != b.len() {
        panic!("Invalid input.");
    }

    a.iter().zip(b.iter()).map(|x| x.0 ^ x.1).collect()
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

    #[test]
    fn fixed_xor_test() {
        assert_eq!(
            fixed_xor(vec![0xde, 0xad, 0xbe, 0xef], vec![0xde, 0xad, 0xbe, 0xef]),
            vec![0x00, 0x00, 0x00, 0x00]
        );

        assert_eq!(
            fixed_xor(vec![0x1c, 0x01, 0x11, 0x00], vec![0x68, 0x69, 0x74, 0x20]),
            vec![0x74, 0x68, 0x65, 0x20]
        );

        assert_eq!(
            fixed_xor(decode(String::from("")), decode(String::from(""))),
            decode(String::from(""))
        );

        assert_eq!(
            fixed_xor(
                decode(String::from("1c0111001f010100061a024b53535009181c")),
                decode(String::from("686974207468652062756c6c277320657965"))
            ),
            decode(String::from("746865206b696420646f6e277420706c6179"))
        );
    }
}
