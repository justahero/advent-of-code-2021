use std::{fmt::Display, ops::Shl};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Operator {
    Sum(Box<Vec<Packet>>),
    Product(Box<Vec<Packet>>),
    Min(Box<Vec<Packet>>),
    Max(Box<Vec<Packet>>),
    /// Greaten Than
    GreaterThan(Box<Vec<Packet>>),
    /// Less Than
    LessThan(Box<Vec<Packet>>),
    /// Equal to
    Equal(Box<Vec<Packet>>),
}

impl Operator {
    pub fn count_version(&self) -> usize {
        let packets = match self {
            Operator::Sum(packets) => packets,
            Operator::Product(packets) => packets,
            Operator::Min(packets) => packets,
            Operator::Max(packets) => packets,
            Operator::GreaterThan(packets) => packets,
            Operator::LessThan(packets) => packets,
            Operator::Equal(packets) => packets,
        };

        packets
            .iter()
            .map(|packet| packet.count_version())
            .sum::<usize>()
    }

    pub fn calculate(&self) -> u32 {
        todo!("needs implementation.")
    }
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u16),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
struct Packet {
    pub version: u16,
    pub type_id: u16,
    pub data: PacketType,
}

impl Packet {
    pub fn literal(version: u16, type_id: u16, literal: u16) -> Self {
        Self {
            version,
            type_id,
            data: PacketType::Literal(literal),
        }
    }

    pub fn operator(version: u16, type_id: u16, operator: Operator) -> Self {
        Self {
            version,
            type_id,
            data: PacketType::Operator(operator),
        }
    }

    pub fn calculate(&self) -> u32 {
        0
    }
}

impl Packet {
    pub fn count_version(&self) -> usize {
        let count = match &self.data {
            PacketType::Operator(operator) => operator.count_version(),
            _ => 0,
        };
        self.version as usize + count as usize
    }
}

/// A basic cursor that reads the binary stream sequentially, handles internal cursor
#[derive(Debug)]
struct BinaryCursor {
    /// Holds all binary data, converted from char, each entry is either '0' or '1'
    pub bytes: Vec<u8>,
    /// Index into the String
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
        self.index >= self.bytes.len() - 1
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
        }
        self.index += bits;

        Ok(result)
    }

    /// Skips the number of bits in the Binary stream
    pub fn skip_bits(&mut self, num_bits: u16) {
        self.index += num_bits as usize;
    }

    /// Forwards the cursor to the next full byte
    pub fn seek_next_byte(&mut self) {
        self.index = ((self.index + 8) / 8) * 8;
    }

    pub fn slice(&self, next_bits: u16) -> &[u8] {
        &self.bytes[self.index..][..next_bits as usize]
    }
}

impl Display for BinaryCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.bytes.iter().map(|x| format!("{:b}", x)).join("");
        write!(f, "Cursor {{ index: {}, bytes: {} }}", self.index, bytes)
    }
}

impl From<&str> for BinaryCursor {
    fn from(input: &str) -> Self {
        let bytes = input
            .chars()
            .filter_map(|b| b.to_digit(2))
            .map(|v| v as u8)
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
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            cursor: BinaryCursor::new(bytes),
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
        Ok(result)
    }

    pub fn slice(&self, next_bits: u16) -> &[u8] {
        self.cursor.slice(next_bits)
    }

    pub fn skip_bits(&mut self, num_bits: u16) {
        self.cursor.skip_bits(num_bits);
    }
}

impl From<&str> for Parser {
    /// Creates a new Parser from a binary String with '0' and '1'
    fn from(input: &str) -> Self {
        Self {
            cursor: BinaryCursor::from(input),
        }
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser {{ {} }}", self.cursor)
    }
}

#[derive(Debug)]
struct BinaryReader {
    input: String,
}

impl BinaryReader {
    /// Creates a new BinaryReader with binary input (a string consisting of '0' and '1')
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn decode(&self) -> Result<Packet, anyhow::Error> {
        let mut parser = Parser::from(self.input.as_str());
        let packet = Self::read_packet(&mut parser)?;
        Ok(packet)
    }

    // Parses the binary input
    fn read_packet(parser: &mut Parser) -> Result<Packet, anyhow::Error> {
        // read packet header
        let (version, id) = parser.read_header()?;

        println!("VERSION: {}, Type ID: {}", version, id);

        let packet = match id {
            4 => Packet::literal(version, id, parser.read_literal()?),
            operator => {
                let packets = Box::new(Self::read_packets(parser)?);
                match operator {
                    0 => Packet::operator(version, id, Operator::Sum(packets)),
                    1 => Packet::operator(version, id, Operator::Product(packets)),
                    2 => Packet::operator(version, id, Operator::Min(packets)),
                    3 => Packet::operator(version, id, Operator::Max(packets)),
                    5 => Packet::operator(version, id, Operator::GreaterThan(packets)),
                    6 => Packet::operator(version, id, Operator::LessThan(packets)),
                    7 => Packet::operator(version, id, Operator::Equal(packets)),
                    _ => panic!("Operator not supported."),
                }
            }
        };

        Ok(packet)
    }

    /// Reads all sub packets, returns the list
    fn read_packets(parser: &mut Parser) -> anyhow::Result<Vec<Packet>> {
        let mode = parser.read_bits(1)?;
        let packets = if mode == 0 {
            let total_length = parser.read_bits(15)?;
            // let packets_parser = Parser { cursor: };
            println!("Total length: {}", total_length);

            // parse the next number of bits until total length is exhausted
            let mut sub_parser = Parser::new(parser.slice(total_length));
            println!("SUB PARSER: {}", sub_parser);

            let mut result = Vec::new();
            while !sub_parser.is_empty() {
                let packet = Self::read_packet(&mut sub_parser)?;
                result.push(packet);
            }
            parser.skip_bits(total_length);

            result
        } else {
            let num_packets = parser.read_bits(11)?;

            println!("Num packets: {}", num_packets);
            let mut sub_packets = Vec::new();
            for _ in 0..num_packets {
                let packet = Self::read_packet(parser)?;
                sub_packets.push(packet);
            }
            sub_packets
        };
        Ok(packets)
    }
}

/// Parses the hexadecimal input string, converts it to binary string
/// then creates a new BinaryReader to parse all binary data.
fn parse_hex_input(hexadecimal: &str) -> BinaryReader {
    let input = hexadecimal
        .chars()
        .filter_map(|c| c.to_digit(16))
        .map(|value| format!("{:04b}", value))
        .collect_vec()
        .join("");
    BinaryReader::new(input)
}

fn main() -> anyhow::Result<()> {
    let reader = parse_hex_input(include_str!("input.txt"));

    let packet = reader.decode()?;
    dbg!(packet.count_version());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_hex_input, BinaryCursor, Operator, Packet, Parser};

    #[test]
    fn check_cursor_read_bits() -> anyhow::Result<()> {
        let input = "110100101111111000101000";
        let mut cursor = BinaryCursor::from(input);
        assert_eq!(0b110, cursor.read_bits(3)?);
        assert_eq!(0b100, cursor.read_bits(3)?);
        assert_eq!(0b10111, cursor.read_bits(5)?);
        assert_eq!(0b11110, cursor.read_bits(5)?);
        assert_eq!(0b00101, cursor.read_bits(5)?);
        assert_eq!(0b000, cursor.read_bits(3)?);
        Ok(())
    }

    #[test]
    fn check_parse_literal() -> anyhow::Result<()> {
        // 5 bits for each literal packet, 1 indicating to continue, 0 the last packet
        let input = "1101100011";
        let mut parser = Parser::from(input);
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
    fn decodes_literal_from_hex_input() -> anyhow::Result<()> {
        let reader = parse_hex_input("D2FE28");
        assert_eq!("110100101111111000101000", reader.input);

        let packet = reader.decode()?;
        assert_eq!(Packet::literal(6, 4, 2021), packet);
        Ok(())
    }

    #[test]
    fn decode_operator_packet_with_two_literals() -> anyhow::Result<()> {
        let reader = parse_hex_input("38006F45291200");
        let expected = Packet::operator(
            1,
            6,
            Operator::LessThan(Box::new(vec![
                Packet::literal(6, 4, 10),
                Packet::literal(2, 4, 20),
            ])),
        );

        assert_eq!(expected, reader.decode()?);
        Ok(())
    }

    #[test]
    fn decode_operator_with_three_subpackets() -> anyhow::Result<()> {
        let reader = parse_hex_input("EE00D40C823060");
        let expected = Packet::operator(
            7,
            3,
            Operator::Max(Box::new(vec![
                Packet::literal(2, 4, 1),
                Packet::literal(4, 4, 2),
                Packet::literal(1, 4, 3),
            ])),
        );

        assert_eq!(expected, reader.decode()?);
        Ok(())
    }

    #[test]
    fn count_versions_in_transmissions() -> anyhow::Result<()> {
        assert_eq!(
            16,
            parse_hex_input("8A004A801A8002F478")
                .decode()?
                .count_version()
        );
        assert_eq!(
            12,
            parse_hex_input("620080001611562C8802118E34")
                .decode()?
                .count_version()
        );
        assert_eq!(
            23,
            parse_hex_input("C0015000016115A2E0802F182340")
                .decode()?
                .count_version()
        );
        assert_eq!(
            31,
            parse_hex_input("A0016C880162017C3686B18A3D4780")
                .decode()?
                .count_version()
        );
        Ok(())
    }

    #[test]
    fn calculate_value_of_transmissions() -> anyhow::Result<()> {
        assert_eq!(3, parse_hex_input("C200B40A82").decode()?.calculate());
        assert_eq!(54, parse_hex_input("04005AC33890").decode()?.calculate());
        assert_eq!(7, parse_hex_input("880086C3E88112").decode()?.calculate());
        assert_eq!(9, parse_hex_input("CE00C43D881120").decode()?.calculate());
        assert_eq!(1, parse_hex_input("D8005AC2A8F0").decode()?.calculate());
        assert_eq!(0, parse_hex_input("F600BC2D8F").decode()?.calculate());
        assert_eq!(0, parse_hex_input("9C005AC2F8F0").decode()?.calculate());
        assert_eq!(
            1,
            parse_hex_input("9C0141080250320F1802104A08")
                .decode()?
                .calculate()
        );

        Ok(())
    }
}
