use crate::{my_base64, hex};

pub fn hex_to_base64(input: String) -> String {
    let input_bytes = hex::decode(input);
    my_base64::encode(input_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64_test() {
        assert_eq!(hex_to_base64(String::from("4d616e")), String::from("TWFu"));
        assert_eq!(hex_to_base64(String::from("4d61")), String::from("TWE="));
        assert_eq!(hex_to_base64(String::from("4d")), String::from("TQ=="));
        assert_eq!(
            hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        );
    }
}
