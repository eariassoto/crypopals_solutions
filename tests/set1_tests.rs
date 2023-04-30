use cryptopals::{cipher_break::break_single_byte, conv::hex_to_base64, hex, xor_pad::XorPad};

#[test]
fn can_convert_hex_to_base64() {
    assert_eq!(
        String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
        hex_to_base64(String::from(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f\
            7573206d757368726f6f6d"
        ))
    );
}

#[test]
fn can_do_fixed_xor() {
    assert_eq!(
        hex::decode(b"746865206b696420646f6e277420706c6179"),
        hex::decode(b"1c0111001f010100061a024b53535009181c")
            .pad_with_key(&hex::decode(b"686974207468652062756c6c277320657965"))
    );
}

#[test]
fn can_break_single_byte_cipher() {
    let input = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let result = break_single_byte(hex::decode(&input));
    assert_eq!(
        Some((0x58, String::from("Cooking MC's like a pound of bacon"))),
        result
    );
}

#[test]
fn can_apply_repeating_key_cipher() {
    let key = b"ICE";
    let input = b"Burning 'em, if you ain't quick and nimble\
    \nI go crazy when I hear a cymbal";

    let result = input.as_slice().pad_with_key(&key.as_slice());
    assert_eq!(input.len(), result.len());

    let expected_result = hex::decode(
        b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a2622632427\
        2765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326\
        302e27282f",
    );

    assert_eq!(expected_result, result);
}

#[test]
fn can_detect_single_byte_cipher() {
    let inputs = include_str!("cipher_data.txt");
    let match_found = inputs
        .lines()
        .filter_map(|input| break_single_byte(hex::decode(&input)))
        .find(|result| *result == (0x35, String::from("Now that the party is jumping\n")));

    assert!(match_found.is_some());
}
