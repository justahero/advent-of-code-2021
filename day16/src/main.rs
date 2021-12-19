use std::ops::Shl;

use itertools::Itertools;

/// A basic cursor that reads the binary stream sequentially, handles internal cursor
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

    // 11010010_11111110_00101000
    pub fn read_bits(&mut self, bits: usize) -> u16 {
        assert!(bits <= 16);

        // TODO refactor later, it's a bit cluttered
        let mut result = 0_u16;
        for i in 0..bits {
            let byte_index = (self.index + i) / 8;
            let bit_index = 7 - (self.index + i) % 8;

            let is_set = self.bytes[byte_index] & 1_u8.shl(bit_index as u32) > 0;
            if is_set {
                result = result | 1_u16.shl(bits - 1 - i);
            }

            // println!("  byte: {}, bit: {} = {}", byte_index, bit_index, is_set);
        }
        self.index += bits;

        println!("Read {} bits: {:0width$b}", bits, result, width = bits);

        result
    }

    /// Forwards the cursor to the next full byte
    pub fn seek_next_byte(&mut self) {
        self.index = (self.index / 8 + 1) * 8;
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

/// Parser struct is to read specific elements from the binary stream
#[derive(Debug)]
struct Parser {
    pub cursor: BinaryCursor,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self { cursor: BinaryCursor::from(input) }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.is_empty()
    }

    pub fn read_bits(&mut self, bits: usize) -> u16 {
        self.cursor.read_bits(bits)
    }

    pub fn read_header(&mut self) -> (u16, u16) {
        let version = self.cursor.read_bits(3);
        let type_id = self.cursor.read_bits(3);
        (version, type_id)
    }

    /// Reads the literal in 5 bits chunk until completes.
    pub fn read_literal(&mut self) -> u16 {
        let mut result = 0_u16;
        loop {
            let bits = self.cursor.read_bits(5);
            result = result.shl(4) + bits & 0xF as u16;
            if bits & 0b10000 >= 1 {
                break;
            }
        }
        self.cursor.seek_next_byte();
        result
    }
}

#[derive(Debug)]
struct BinaryReader {
    input: String,
}

impl BinaryReader {
    const LITERAL: u16 = 0b100;

    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    pub fn decode(&self) -> Result<(), anyhow::Error> {
        let mut cursor = Parser::new(self.input.as_str());
        while !cursor.is_empty() {
            Self::parse(&mut cursor)?;
        }
        Ok(())
    }

    // Parses the binary input
    fn parse(parser: &mut Parser) -> Result<(), anyhow::Error> {
        // read packet header
        let (version, type_id) = parser.read_header();

        println!("VERSION: {}, Type ID: {}", version, type_id);

        match type_id {
            Self::LITERAL => {
                // parse literal
                let literal = parser.read_literal();
                println!("Literal: {}", literal);
            }
            _ => {
                // read operator and sub packets
                let mode = parser.read_bits(1);
                if mode == 0 {
                    let total_length = parser.read_bits(15);
                    println!("Total length: {}", total_length);
                } else {
                    let num_packets = parser.read_bits(11);
                    println!("Num packets: {}", num_packets);
                }
            }
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let reader = BinaryReader::new(include_str!("input.txt"));
    reader.decode()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{BinaryCursor, BinaryReader, Parser};

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
    fn check_parse_literal() -> Result<(), anyhow::Error> {
        let input = "11011000";
        let mut parser = Parser::new(input);
        let literal = parser.read_literal();
        assert_eq!(0b1011, literal);
        Ok(())
    }

    #[test]
    fn check_parse_cursor_from_string() {
        let cursor = BinaryCursor::from("110100101111111000101000");
        assert_eq!(vec![0b11010010_u8, 0b11111110, 0b00101000], cursor.bytes);
    }

    #[test]
    fn decode_input_string() {
        let reader = BinaryReader::new("110100101111111000101000");
        assert!(reader.decode().is_ok());
    }

    #[test]
    fn decodes_input_with_operator() {
        let input = "00111000000000000110111101000101001010010001001000000000";
        let reader = BinaryReader::new(input);
        assert!(reader.decode().is_ok());
    }
}
