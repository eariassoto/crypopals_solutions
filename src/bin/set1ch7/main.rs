
use std::io::Read;

use aes::Aes128;
use aes::cipher::{ BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use base64::Engine;
use base64::engine::general_purpose;

fn main(){
    let key: &[u8; 16] = b"YELLOW SUBMARINE";
    let input: String = include_str!("encrypted_file.txt")
        .chars()
        .filter(|&c| c != '\n')
        .collect();
    let input = general_purpose::STANDARD.decode(input).unwrap();

    let cipher = Aes128::new_from_slice(key).unwrap();

    let mut decrypted_blocks = Vec::new();
    for block in input.as_slice().chunks_exact(16) {
        let mut block_array = GenericArray::clone_from_slice(block);
        cipher.decrypt_block(&mut block_array);
        decrypted_blocks.extend_from_slice(&block_array);
    }

    let decrypted_message = String::from_utf8(decrypted_blocks).unwrap();
    println!("{}", decrypted_message);
}