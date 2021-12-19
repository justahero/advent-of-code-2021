use std::ops::Shl;

use itertools::Itertools;

#[derive(Debug)]
struct BinaryCursor {
    /// Holds all binary data
    bytes: Vec<u8>,
    /// Index into the bit array
    index: usize,
}

impl<'a> BinaryCursor {
    pub fn new(bytes: &[u8]) -> Self {
        Self { bytes: bytes.iter().cloned().collect_vec(), index: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.index / 8 >= self.bytes.len()
    }

    pub fn read_bits(&mut self, bits: usize) -> u8 {
        assert!(bits <= 8);

        // TODO refactor later, it's a bit cluttered
        let mut result = 0;
        for i in 0..bits {
            let byte_index = (self.index + i) / 8;
            let bit_index = 7 - (self.index + i) % 8;

            let is_set = self.bytes[byte_index] & 1_u8.shl(bit_index as u32) > 0;
            if is_set {
                result = result | 1_u8.shl(bits - 1 - i);
            }
        }
        self.index += bits;

        result
    }

    /// Forwards the cursor to the next full byte
    pub fn seek_next_byte(&mut self) {
        self.index = (self.index / 8 + 1) * 8;
    }
}

#[derive(Debug)]
struct BinaryReader {
    input: String,
}

impl BinaryReader {
    const LITERAL: u8 = 0b100;

    pub fn new(input: &str) -> Self {
        Self { input: input.to_string() }
    }

    pub fn decode(&self) {
        let mut cursor = BinaryCursor::new(&self.input.as_bytes()[..]);

        while !cursor.is_empty() {
            // read packet header
            let version = cursor.read_bits(3);
            let typeId = cursor.read_bits(3);

            match typeId {
                Self::LITERAL => {
                    // parse literal
                    
                },
                _ => panic!(),
            }
        }
    }
}

fn main() {
    let _reader = BinaryReader::new(include_str!("input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::BinaryCursor;

    #[test]
    fn check_cursor_read_bits() {
        let input = &[0b11010010_u8, 0b11111110, 0b00101000];
        let mut cursor = BinaryCursor::new(&input[..]);
        assert_eq!(0b110, cursor.read_bits(3));
        assert_eq!(0b100, cursor.read_bits(3));
        assert_eq!(0b10111, cursor.read_bits(5));
        assert_eq!(0b11110, cursor.read_bits(5));
        assert_eq!(0b00101, cursor.read_bits(5));
        assert_eq!(0b000, cursor.read_bits(3));
    }

    #[test]
    fn check_forward_byte() {
        let input = &[0b11010000_u8, 0b11000000];
        let mut cursor=  BinaryCursor::new(&input[..]);
        assert_eq!(0b1101, cursor.read_bits(4));
        cursor.seek_next_byte();
        assert_eq!(0b1100, cursor.read_bits(4));
    }
}
