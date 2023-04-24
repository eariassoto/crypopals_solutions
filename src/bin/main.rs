use std::cmp::Ordering;

use cryptopals::{hex, xor::Xor};

fn get_letter_frequency(input: &[u8]) -> [f64; 26] {
    let mut res: [f64; 26] = [0.0; 26];
    let mut total_letters = 0;
    for byte in input {
        match *byte {
            b'a'..=b'z' => {
                res[(*byte - b'a') as usize] += 1.0;
                total_letters += 1;
            }
            b'A'..=b'Z' => {
                res[(*byte - b'A') as usize] += 1.0;
                total_letters += 1;
            }
            _ => {}
        }
    }

    // Normalize the observed frequencies by the total number of letters
    for freq in &mut res {
        *freq /= total_letters as f64;
    }
    res
}

#[derive(Debug, PartialEq)]
struct Score {
    letter: char,
    message: String,
    sum_errors: f64,
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sum_errors.partial_cmp(&other.sum_errors)
    }
}

fn get_top_scores(input: &String) -> Vec<Score> {
    ('A'..='Z')
        .map(|letter| {
            let xored = hex::decode(&input).xor_byte(letter as u8);

            // create a letter frequency map with the top letters
            let letter_freq = get_letter_frequency(&xored);

            // to give some score: compare the frequency to the expected english language frequencies.

            // Frequencies based on the Corpus of Contemporary American English (COCA) data set.
            let expected_freqs = [
                0.0817, 0.0149, 0.0278, 0.0425, 0.127, 0.0223, 0.0202, 0.0609, 0.0697, 0.0015,
                0.0077, 0.0403, 0.0241, 0.0675, 0.0751, 0.0193, 0.0009, 0.0599, 0.0633, 0.0906,
                0.0276, 0.0098, 0.0236, 0.0015, 0.0197, 0.0007,
            ];

            let sum_errors = letter_freq
                .iter()
                .zip(expected_freqs.iter())
                .fold(0.0, |acc, (act, exp)| acc + (*act - *exp).abs());

            Score {
                letter: letter,
                message: String::from_utf8(xored).unwrap_or_default(),
                sum_errors: sum_errors,
            }
        })
        .collect()
}

fn main() {
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    //Calculate scores for each letter
    let mut scores = get_top_scores(&input);
    scores.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    for score in scores.iter().take(3) {
        println!("{:?}", score);
    }
}
