use cryptopals::{hex, xor_pad::XorPad, text_score::score_text};

fn main() {
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let all_ciphers = (0u8..=0xffu8).map(|b| (b, hex::decode(&input).pad_with_byte(b)));
    let mut valid_ascii_ciphers = all_ciphers
        .filter(|(_, cipher)| {
            !cipher
                .iter()
                .any(|&c| !c.is_ascii() || (c.is_ascii_control() && c != b'\n'))
        })
        .map(|(key, cipher)| {
            let (cipher, score) = score_text(String::from_utf8(cipher).unwrap());
            (key, cipher, score)
        })
        .collect::<Vec<(u8, String, f64)>>();

    for (key, cipher, score) in &valid_ascii_ciphers {
        println!("[{:#04x}]. Cipher: {:?}. Score: {:?}", key, cipher, score,);
    }

    valid_ascii_ciphers.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

    match valid_ascii_ciphers.first() {
        Some((key, cipher, score)) => println!(
            "Match found for key: [{:#04x}]. Cipher: {:?}. Score: {:?}",
            key, cipher, score,
        ),
        None => {}
    };
}
