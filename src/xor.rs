pub trait Xor<T: AsRef<[u8]>> {
    fn xor_byte(&self, b: u8) -> Vec<u8>;
    fn xor_pad(&self, other: &T) -> Vec<u8>;
}

impl<T: AsRef<[u8]>> Xor<T> for T {
    fn xor_byte(&self, b: u8) -> Vec<u8> {
        self.as_ref().iter().map(|x| x ^ b).collect()
    }

    fn xor_pad(&self, other: &T) -> Vec<u8> {
        let a = self.as_ref();
        let b = other.as_ref();
        if a.len() != b.len() {
            panic!("Invalid input.");
        }
    
        a.iter().zip(b.iter()).map(|x| x.0 ^ x.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_byte_test() {
        let input: Vec<u8> = vec![0b00000000, 0b01010101, 0b11110000];
        let expected = vec![0b11111111, 0b10101010, 0b00001111];
        assert_eq!(expected, input.xor_byte(0b11111111));
    }

    #[test]
    fn xor_pad_test() {
        let a: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
        let b: Vec<u8> =  vec![0xde, 0xad, 0xbe, 0xef];
        let expected: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        assert_eq!(expected, a.xor_pad(&b));

        let a: Vec<u8> = vec![0x1c, 0x01, 0x11, 0x00];
        let b: Vec<u8> =  vec![0x68, 0x69, 0x74, 0x20];
        let expected: Vec<u8> = vec![0x74, 0x68, 0x65, 0x20];
        assert_eq!(expected, a.xor_pad(&b));

        let a: Vec<u8> = vec![];
        let b: Vec<u8> =  vec![];
        let expected: Vec<u8> = vec![];
        assert_eq!(expected, a.xor_pad(&b));
    }
}
