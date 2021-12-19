use std::{fmt::Display, ops::Shl};

use itertools::Itertools;

/// A basic cursor that reads the binary stream sequentially, handles internal cursor
#[derive(Debug)]
struct BinaryCursor {
    /// Holds all binary data, converted from char, each entry is either '0' or '1'
    pub bytes: Vec<u8>,
    /// Index into the String
    index: usize,
}

impl<'a> BinaryCursor {
    pub fn new(bytes: String) -> Self {
        let bytes = bytes
            .chars()
            .filter_map(|b| b.to_digit(2))
            .map(|v| v as u8)
            .collect_vec();

        Self { bytes, index: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= (self.bytes.len() - 1)
    }

    // 11010010_11111110_00101000
    pub fn read_bits(&mut self, bits: usize) -> anyhow::Result<u16> {
        assert!(bits <= 16);

        // TODO refactor later, it's a bit cluttered
        let mut result = 0_u16;
        for i in 0..bits {
            let byte_index = self.index + i;

            // parse current char
            let value = self.bytes[byte_index];
            if value == 1 {
                result = result | 1_u16.shl(bits - 1 - i);
            }

            // println!("  byte: {}, bit: {} = {}", byte_index, bit_index, is_set);
        }
        self.index += bits;

        // println!("Read {} bits: {:0width$b}, index: {}", bits, result, self.index, width = bits);

        Ok(result)
    }

    /// Forwards the cursor to the next full byte
    pub fn seek_next_byte(&mut self) {
        self.index = ((self.index + 8) / 8) * 8;
    }
}

impl Display for BinaryCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:b}", byte)?;
        }
        Ok(())
    }
}

impl From<&str> for BinaryCursor {
    fn from(input: &str) -> Self {
        Self::new(input.to_string())
    }
}

/// Parser struct is to read specific elements from the binary stream
#[derive(Debug)]
struct Parser {
    pub cursor: BinaryCursor,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            cursor: BinaryCursor::from(input),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.is_empty()
    }

    pub fn read_bits(&mut self, bits: usize) -> anyhow::Result<u16> {
        self.cursor.read_bits(bits)
    }

    pub fn read_header(&mut self) -> anyhow::Result<(u16, u16)> {
        let version = self.cursor.read_bits(3)?;
        let type_id = self.cursor.read_bits(3)?;
        Ok((version, type_id))
    }

    /// Reads the literal in 5 bits chunk until completes.
    /// TODO refactor this logic, it's a bit cluttered
    pub fn read_literal(&mut self) -> anyhow::Result<u16> {
        let mut result = 0_u16;
        loop {
            let bits = self.cursor.read_bits(5)?;
            result = result.shl(4) + (bits & 0xF as u16);
            if bits & 0b10000 >= 1 {
                continue;
            }
            break;
        }
        self.cursor.seek_next_byte();
        Ok(result)
    }
}

#[derive(Debug)]
struct BinaryReader {
    input: String,
}

impl BinaryReader {
    const LITERAL: u16 = 0b100;

    /// Creates a new BinaryReader with binary input (a string consisting of '0' and '1')
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn decode(&self) -> Result<(), anyhow::Error> {
        let mut parser = Parser::new(self.input.as_str());
        while !parser.is_empty() {
            Self::read_packet(&mut parser)?;
        }
        Ok(())
    }

    // Parses the binary input
    fn read_packet(parser: &mut Parser) -> Result<(), anyhow::Error> {
        // read packet header
        let (version, type_id) = parser.read_header()?;

        println!("VERSION: {}, Type ID: {}", version, type_id);

        match type_id {
            Self::LITERAL => {
                let literal = parser.read_literal()?;
                println!("Literal: {}", literal);
            }
            _ => {
                // read operator and sub packets
                let mode = parser.read_bits(1)?;
                if mode == 0 {
                    let total_length = parser.read_bits(15)?;
                    // let packets_parser = Parser { cursor: };
                    println!("Total length: {}", total_length);
                } else {
                    let num_packets = parser.read_bits(11)?;
                    println!("Num packets: {}", num_packets);
                }
            }
        }

        Ok(())
    }
}

fn parse_hex_input(hexadecimal: &str) -> BinaryReader {
    let input = hexadecimal
        .chars()
        .filter_map(|c| c.to_digit(16))
        .map(|value| format!("{:08b}", value))
        .collect_vec()
        .join("");
    BinaryReader::new(input)
}

fn main() -> anyhow::Result<()> {
    let reader = parse_hex_input(include_str!("input.txt"));
    reader.decode()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{BinaryCursor, BinaryReader, Parser, parse_hex_input};

    #[test]
    fn check_cursor_read_bits() -> anyhow::Result<()> {
        let input = "110100101111111000101000";
        let mut cursor = BinaryCursor::new(input.to_string());
        assert_eq!(0b110, cursor.read_bits(3)?);
        assert_eq!(0b100, cursor.read_bits(3)?);
        assert_eq!(0b10111, cursor.read_bits(5)?);
        assert_eq!(0b11110, cursor.read_bits(5)?);
        assert_eq!(0b00101, cursor.read_bits(5)?);
        assert_eq!(0b000, cursor.read_bits(3)?);
        Ok(())
    }

    #[test]
    fn check_forward_byte() -> anyhow::Result<()> {
        let input = "1101000011000000";
        let mut cursor = BinaryCursor::new(input.to_string());
        assert_eq!(0b1101, cursor.read_bits(4)?);
        cursor.seek_next_byte();
        assert_eq!(0b1100, cursor.read_bits(4)?);
        Ok(())
    }

    #[test]
    fn check_parse_literal() -> anyhow::Result<()> {
        // 5 bits for each literal packet, 1 indicating to continue, 0 the last packet
        let input = "1101100011";
        let mut parser = Parser::new(input);
        let literal = parser.read_literal()?;
        assert_eq!(0b10110011, literal);
        Ok(())
    }

    #[test]
    fn check_parse_cursor_from_string() {
        let cursor = BinaryCursor::from("110100101111111000101000");
        assert_eq!(
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
            cursor.bytes
        );
    }

    #[test]
    fn decode_binary_input_example() {
        let input = "00111000000000000110111101000101001010010001001000000000";
        let reader = BinaryReader::new(input.to_string());
        assert!(reader.decode().is_ok());
    }

    #[test]
    fn decode_hexadecimal_examples() {
        let reader = parse_hex_input("8A004A801A8002F478");
        assert!(reader.decode().is_ok());
    }
}
