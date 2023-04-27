/// Trait for objects that can perform XOR padding on a byte array using a specified byte or key.
pub trait XorPad<T>
where
    T: AsRef<[u8]>,
{
    /// Returns a new byte array XOR padded with the specified byte key.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the object performing the padding.
    /// * `byte` - The key to pad the byte array.
    ///
    /// # Returns
    ///
    /// A new byte array padded with the specified byte.
    fn pad_with_byte(&self, byte: u8) -> Vec<u8>;

    /// Returns a new byte array XOR padded with the specified key.
    ///
    /// If the key is smaller than the original byte array, it will be applied repeatedly in a cyclic manner.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the object performing the padding.
    /// * `key` - A reference to the key to use for XOR padding.
    ///
    /// # Returns
    ///
    /// A new byte array padded with the XOR of the original byte array and the specified key.
    fn pad_with_key(&self, key: &T) -> Vec<u8>;
}

impl<T: AsRef<[u8]>> XorPad<T> for T {
    fn pad_with_byte(&self, byte: u8) -> Vec<u8> {
        self.as_ref().iter().map(|x| x ^ byte).collect()
    }

    fn pad_with_key(&self, key: &T) -> Vec<u8> {
        let key = key.as_ref();
        if key.is_empty() {
            panic!("Key cannot be empty.");
        }

        self.as_ref()
            .iter()
            .zip(key.iter().cycle())
            .map(|x| x.0 ^ x.1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_with_byte_test() {
        let input: Vec<u8> = vec![0b00000000, 0b01010101, 0b11110000];
        let expected = vec![0b11111111, 0b10101010, 0b00001111];
        assert_eq!(expected, input.pad_with_byte(0b11111111));
    }

    #[test]
    fn pad_with_key_test() {
        let text: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
        let key: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
        let expected: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        assert_eq!(expected, text.pad_with_key(&key));

        let text: Vec<u8> = vec![0x1c, 0x01, 0x1c, 0x00];
        let key: Vec<u8> = vec![0x68, 0x69, 0x74, 0x20];
        let expected: Vec<u8> = vec![0x74, 0x68, 0x68, 0x20];
        assert_eq!(expected, text.pad_with_key(&key));

        let key: Vec<u8> = vec![0x1c, 0xff];
        let expected: Vec<u8> = vec![0x00, 0xfe, 0x00, 0xff];
        assert_eq!(expected, text.pad_with_key(&key));
    }

    #[test]
    #[should_panic(expected = "Key cannot be empty.")]
    fn pad_with_key_invalid_key_test() {
        let text: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
        let key: Vec<u8> = vec![];
        text.pad_with_key(&key);
    }
}
