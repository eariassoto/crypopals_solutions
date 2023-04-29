use crate::error_metrics::ErrorMetrics;

// Frequencies (a-z) based on the Corpus of Contemporary American English (COCA) data set.
const EXPECTED_FREQUENCIES: [f64; 26] = [
    0.0817, 0.0149, 0.0278, 0.0425, 0.127, 0.0223, 0.0202, 0.0609, 0.0697, 0.0015, 0.0077, 0.0403,
    0.0241, 0.0675, 0.0751, 0.0193, 0.0009, 0.0599, 0.0633, 0.0906, 0.0276, 0.0098, 0.0236, 0.0015,
    0.0197, 0.0007,
];

fn get_letter_frequency(input: &String) -> [f64; 26] {
    let mut res: [f64; 26] = [0.0; 26];
    let mut total_letters = 0;
    for ch in input.chars() {
        if ch.is_ascii_alphabetic() {
            res[(ch.to_ascii_lowercase() as u8 - b'a') as usize] += 1.0;
            total_letters += 1;
        }
    }

    // Normalize the observed frequencies by the total number of letters
    for freq in &mut res {
        *freq /= total_letters as f64;
    }
    res
}

fn average_word_length(s: &String) -> Option<f64> {
    let words: Vec<&str> = s.split_whitespace().collect();

    if words.len() == 0 {
        return None;
    }

    let total_length: usize = words.iter().map(|w| w.len()).sum();
    let avg_length = total_length as f64 / words.len() as f64;

    Some(avg_length)
}

fn calculate_score(value1: f64, value2: f64) -> f64 {
    // Transform the values using the scaled inverse hyperbolic tangent function
    let normalized_value1 = f64::tanh(value1 / 2.0);
    let normalized_value2 = f64::tanh(value2 / 2.0);

    // Calculate the weighted sum
    let weighted_sum = (0.6 * normalized_value1) + (0.4 * normalized_value2);

    // Apply a sigmoid function to map the result to a range between 0 and 1
    1.0 / (1.0 + (-weighted_sum).exp())
}

pub fn score_text(input: String) -> (String, f64) {
    let score_letter_freq = EXPECTED_FREQUENCIES
        .mean_absolute_error(&get_letter_frequency(&input))
        .unwrap();
    let score_average_word_len = (5.0 - average_word_length(&input).unwrap()).abs();

    (
        input,
        calculate_score(score_letter_freq, score_average_word_len),
    )
}
