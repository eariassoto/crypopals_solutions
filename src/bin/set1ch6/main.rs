use base64::{engine::general_purpose, Engine as _};
use cryptopals::{cipher_break::break_single_byte, xor_pad::{hamming_distance, XorPad}};
use itertools::Itertools;

fn get_possible_key_sizes(input: &Vec<u8>) -> Vec<usize> {
    (2..=40)
        .sorted_by_key(|&s| {
            let samples = 4;
            let combinations = input.chunks(s).take(samples).combinations(2);
            let distances_avg: f64 = combinations
                .map(|comb| hamming_distance(comb[0], comb[1]) as f64 / s as f64)
                .sum::<f64>()
                / samples as f64;
            (100f64 * distances_avg) as u32
        })
        .take(4)
        .collect()
}

fn get_data_transposed(key_size: usize, input: &Vec<u8>) -> Vec<Vec<u8>> {
    let blocks: Vec<&[u8]> = input.chunks(key_size).collect();
    (0..key_size)
        .map(|b| {
            blocks.iter().fold(vec![], |mut acc, &chunk| {
                if b < chunk.len() {
                    acc.push(chunk[b]);
                }
                acc
            })
        })
        .collect()
}

fn main() {
    let input: String = include_str!("encrypted_file.txt")
        .chars()
        .filter(|&c| c != '\n')
        .collect();
    let input = general_purpose::STANDARD.decode(input).unwrap();
    println!("Finding key sizes for input...");

    let key_sizes = get_possible_key_sizes(&input);
    println!("Found possible key sizes: {:?}.", key_sizes);

    for ks in key_sizes {
        println!("Trying {:?} as key size.", ks);

        let chunks = get_data_transposed(ks, &input);
        let keys_for_chunks: Vec<(u8, String)> =
            chunks.iter().filter_map(|c| break_single_byte(c)).collect();
        if keys_for_chunks.len() == ks {
            println!("Found chunk keys using key size {:?}.", ks);

            let key = keys_for_chunks.iter().fold(vec![], |mut acc, x| {
                acc.push(x.0);
                acc
            });
            println!("Encryption key is: {:?}.", std::str::from_utf8(&key).unwrap());

            let unencrypted_file = input.pad_with_key(&key);
            println!("Unencrypted file contents:\n{}.", std::str::from_utf8(&unencrypted_file).unwrap());
            break;
        }
    }

}
