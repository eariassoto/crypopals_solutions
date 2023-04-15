pub fn decode(input: String) -> Vec<u8> {
    // TODO: handle errors properlz
    if input.len() % 2 == 1 {
        panic!("Invalid input.")
    }

    if input.starts_with("0x") || input.starts_with("0X") {
        2..input.len()
    } else {
        0..input.len()
    }
    .step_by(2)
    .map(|i| match u8::from_str_radix(&input[i..i + 2], 16) {
        Ok(res) => res,
        Err(e) => panic!("Invalid hex pair value. Error: {:?}", e),
    })
    .collect()
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
