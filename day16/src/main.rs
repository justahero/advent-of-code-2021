use std::ops::Shl;

use itertools::Itertools;

#[derive(Debug)]
struct BinaryCursor {
    /// Holds all binary data
    pub bytes: Vec<u8>,
    /// Index into the bit array
    index: usize,
}

impl<'a> BinaryCursor {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.iter().cloned().collect_vec(),
            index: 0,
        }
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

    /// Reads the 5 bit literal, returns a tuple containing a bool to indicate to continue reading the literal
    /// and 4 bits that make up the literal.
    pub fn read_literal(&mut self) -> (bool, u8) {
        let bits = self.read_bits(5);
        (bits & 0b10000 > 1, bits & 0xF)
    }
}

impl From<&str> for BinaryCursor {
    fn from(input: &str) -> Self {
        let bytes = input
            .chars()
            .chunks(8)
            .into_iter()
            .map(|chunk| {
                u8::from_str_radix(&chunk.into_iter().collect::<String>(), 2).unwrap()
            })
            .collect_vec();
        Self::new(&bytes)
    }
}

#[derive(Debug)]
struct BinaryReader {
    input: String,
}

impl BinaryReader {
    const LITERAL: u8 = 0b100;

    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    pub fn decode(&self) {
        let mut cursor = BinaryCursor::new(&self.input.as_bytes()[..]);

        while !cursor.is_empty() {
            // read packet header
            let version = cursor.read_bits(3);
            let type_id = cursor.read_bits(3);

            match type_id {
                Self::LITERAL => {
                    // parse literal
                    let mut result = 0_u64;
                    loop {
                        let (next, literal) = cursor.read_literal();
                        result = result.shl(4) + literal as u64;
                        if !next {
                            break;
                        }
                    }
                    println!("Literal: {}", result);
                }
                _ => panic!(),
            }
        }
    }
}

fn main() {
    let reader = BinaryReader::new(include_str!("input.txt"));
    reader.decode();
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
        let mut cursor = BinaryCursor::new(&input[..]);
        assert_eq!(0b1101, cursor.read_bits(4));
        cursor.seek_next_byte();
        assert_eq!(0b1100, cursor.read_bits(4));
    }

    #[test]
    fn check_read_literal() {
        let input = &[0b11011000];
        let mut cursor = BinaryCursor::new(&input[..]);
        let (next, literal) = cursor.read_literal();
        assert!(next);
        assert_eq!(0b1011, literal);
    }

    #[test]
    fn check_parse_cursor_from_string() {
        let cursor = BinaryCursor::from("110100101111111000101000");
        assert_eq!(vec![0b11010010_u8, 0b11111110, 0b00101000], cursor.bytes);
    }
}
