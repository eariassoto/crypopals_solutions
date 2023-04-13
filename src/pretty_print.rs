pub fn ascii_hex_to_bytes(input: String) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| match u8::from_str_radix(&input[i..i + 2], 16) {
            Ok(res) => res,
            Err(e) => panic!("Invalid hex pair value. Error: {:?}", e),
        })
        .collect()
}

pub fn bytes_base64_to_ascii(input: Vec<u8>, padding: bool) -> String {
    let res = input.iter().map(base64_as_ascii).collect::<String>();

    // Pad the ascii output if needed
    let last_chunk = res.len() % 4;
    if padding && last_chunk > 0 {
        // Should be ok as the + operator will steal the buffer from left side
        res + &str::repeat("=", 4 - last_chunk)
    } else {
        res
    }
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
