use cryptopals::{hex, rmse::Rmse, xor::Xor};

// Frequencies (a-z) based on the Corpus of Contemporary American English (COCA) data set.
const EXPECTED_FREQUENCIES: [f64; 26] = [
    0.0817, 0.0149, 0.0278, 0.0425, 0.127, 0.0223, 0.0202, 0.0609, 0.0697, 0.0015, 0.0077, 0.0403,
    0.0241, 0.0675, 0.0751, 0.0193, 0.0009, 0.0599, 0.0633, 0.0906, 0.0276, 0.0098, 0.0236, 0.0015,
    0.0197, 0.0007,
];

fn get_letter_frequency(input: &[u8]) -> [f64; 26] {
    let mut res: [f64; 26] = [0.0; 26];
    let mut total_letters = 0;
    for byte in input {
        if byte.is_ascii_alphabetic() {
            res[(byte.to_ascii_lowercase() - b'a') as usize] += 1.0;
            total_letters += 1;
        }
    }

    // Normalize the observed frequencies by the total number of letters
    for freq in &mut res {
        *freq /= total_letters as f64;
    }
    res
}

fn main() {
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let input = hex::decode(&input);
    let result = (0x00..=0xFF)
        .map(|b| input.xor_byte(b as u8))
        .filter(|xored| {
            !xored
                .iter()
                .any(|&c| !c.is_ascii() || (c.is_ascii_control() && c != b'\n'))
        })
        .min_by_key(|input| {
            (EXPECTED_FREQUENCIES
                .root_mean_square_dev(&get_letter_frequency(input))
                .unwrap()
                * 100.0) as u32
        });
    match result {
        Some(r) => println!("Match found: {:?}", std::str::from_utf8(&r).unwrap()),
        None => {}
    };
}
