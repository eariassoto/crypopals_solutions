use crate::error_metrics::ErrorMetrics;

// Frequencies (a-z) based on the Corpus of Contemporary American English (COCA) data set.
const EXPECTED_FREQUENCIES: [f64; 26] = [
    0.0817, 0.0149, 0.0278, 0.0425, 0.127, 0.0223, 0.0202, 0.0609, 0.0697, 0.0015, 0.0077, 0.0403,
    0.0241, 0.0675, 0.0751, 0.0193, 0.0009, 0.0599, 0.0633, 0.0906, 0.0276, 0.0098, 0.0236, 0.0015,
    0.0197, 0.0007,
];

fn get_letter_frequency<T: AsRef<[u8]>>(input: &T) -> [f64; 26] {
    let mut res: [f64; 26] = [0.0; 26];
    let mut total_letters = 0;
    for byte in input.as_ref() {
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

pub fn score_letter_frequency<T: AsRef<[u8]>>(input: &T) -> f64 {
    EXPECTED_FREQUENCIES
        .mean_absolute_error(&get_letter_frequency(input))
        .unwrap()
}
