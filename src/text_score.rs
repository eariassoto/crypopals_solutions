use crate::error_metrics::ErrorMetrics;

// Frequencies (a-z) based on the Corpus of Contemporary American English (COCA) data set.
const LETTER_FREQUENCIES: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
    0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
    0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
];

fn get_letter_frequency(input: &String) -> [f64; 26] {
    let mut res: [f64; 26] = [0.0; 26];
    for ch in input.chars() {
        if ch.is_ascii_alphabetic() {
            res[(ch.to_ascii_lowercase() as u8 - b'a') as usize] += 1.0;
        }
    }

    // Normalize the observed frequencies by the total number of letters
    for freq in &mut res {
        *freq /= input.len() as f64;
    }
    res
}

pub fn score_text(input: String) -> (String, f64) {
    let score_letter_freq = LETTER_FREQUENCIES
        .mean_absolute_error(&get_letter_frequency(&input))
        .unwrap();

    (input, score_letter_freq)
}
