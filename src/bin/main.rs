use cryptopals::{hex, letter_freq::score_letter_frequency, xor_pad::XorPad};

fn main() {
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let all_byte_pads = (0u8..=0xffu8).map(|b| (b, hex::decode(&input).pad_with_byte(b)));
    let valid_ascii_pads = all_byte_pads.filter(|(_, cipher)| {
        !cipher
            .iter()
            .any(|&c| !c.is_ascii() || (c.is_ascii_control() && c != b'\n'))
    });

    match valid_ascii_pads
        .min_by_key(|(_, cipher)| (score_letter_frequency(&cipher) * 100.0) as u32)
    {
        Some((key, cipher)) => println!(
            "Match found for key: [{:#04x}]. Cipher: {:?}",
            key,
            std::str::from_utf8(&cipher).unwrap()
        ),
        None => {}
    };
}
