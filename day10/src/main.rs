use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftArrow,
    RightArrow,
    LeftBracket,
    RightBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl From<char> for Token {
    fn from(val: char) -> Self {
        match val {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '<' => Token::LeftArrow,
            '>' => Token::RightArrow,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            v => panic!("Unknown char '{}' found", v),
        }
    }
}

impl Token {
    pub fn opens(&self) -> bool {
        const TOKENS: [Token; 4] = [
            Token::LeftParen,
            Token::LeftBrace,
            Token::LeftArrow,
            Token::LeftBracket,
        ];
        TOKENS.contains(self)
    }

    pub fn matches(&self, rhs: &Token) -> bool {
        matches!(
            (self, rhs),
            (Token::LeftParen, Token::RightParen)
                | (Token::LeftBrace, Token::RightBrace)
                | (Token::LeftArrow, Token::RightArrow)
                | (Token::LeftBracket, Token::RightBracket)
        )
    }

    pub fn opposite(&self) -> Token {
        match self {
            Token::LeftParen => Token::RightParen,
            Token::RightParen => Token::LeftParen,
            Token::LeftBrace => Token::RightBrace,
            Token::RightBrace => Token::LeftBrace,
            Token::LeftArrow => Token::RightArrow,
            Token::RightArrow => Token::LeftArrow,
            Token::LeftBracket => Token::RightBracket,
            Token::RightBracket => Token::LeftBracket,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Token::LeftParen => '(',
            Token::RightParen => ')',
            Token::LeftBrace => '{',
            Token::RightBrace => '}',
            Token::LeftArrow => '<',
            Token::RightArrow => '>',
            Token::LeftBracket => '[',
            Token::RightBracket => ']',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum DecoderResult {
    Ok,
    Corrupt(Token),
    Incomplete,
}

fn score(lines: Vec<String>) -> u32 {
    let score_table: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();

    let mut score = 0;
    for line in lines {
        if let DecoderResult::Corrupt(token) = decode_chunk(&line) {
            score += score_table[&token.as_char()]
        }
    }
    score
}

/// Decodes the chunk and returns true if pairs match fully
fn decode_chunk(chunk: &str) -> DecoderResult {
    let mut stack = Vec::new();

    for token in chunk.chars().map(Token::from).collect_vec() {
        if token.opens() {
            stack.push(token);
        } else if let Some(last_token) = stack.pop() {
            if !last_token.matches(&token) {
                if stack.is_empty() {
                    return DecoderResult::Incomplete;
                } else {
                    return DecoderResult::Corrupt(token);
                }
            }
        }
    }

    if stack.is_empty() {
        DecoderResult::Ok
    } else {
        DecoderResult::Incomplete
    }
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(String::from)
        .collect_vec()
}

fn main() {
    let chunks = parse_input(include_str!("input.txt"));

    let total = score(chunks);
    dbg!(total);
}

#[cfg(test)]
mod tests {
    use crate::{decode_chunk, parse_input, score, DecoderResult, Token};

    const INPUT: &str = r#"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "#;

    #[test]
    fn test_valid_lines() {
        assert_eq!(DecoderResult::Ok, decode_chunk("[]"));
        assert_eq!(DecoderResult::Ok, decode_chunk("([])"));
        assert_eq!(DecoderResult::Ok, decode_chunk("{()()()}"));
        assert_eq!(DecoderResult::Ok, decode_chunk("<([{}])>"));
        assert_eq!(DecoderResult::Ok, decode_chunk("[<>({}){}[([])<>]]"));
        assert_eq!(DecoderResult::Ok, decode_chunk("(((((((((())))))))))"));
    }

    #[test]
    fn test_incomplete_lines() {
        assert_eq!(DecoderResult::Incomplete, decode_chunk("(]"));
        assert_eq!(DecoderResult::Incomplete, decode_chunk("{()()()>"));
        assert_eq!(DecoderResult::Incomplete, decode_chunk("(((()))}"));
        assert_eq!(DecoderResult::Incomplete, decode_chunk("<([]){()}[{}])"));
    }

    #[test]
    fn test_corrupted_lines() {
        assert_eq!(
            DecoderResult::Corrupt(Token::RightBrace),
            decode_chunk("{([(<{}[<>[]}>{[]{[(<()>")
        );
        assert_eq!(
            DecoderResult::Corrupt(Token::RightParen),
            decode_chunk("[[<[([]))<([[{}[[()]]]")
        );
        assert_eq!(
            DecoderResult::Corrupt(Token::RightBracket),
            decode_chunk("[{[{({}]{}}([{[{{{}}([]")
        );
        assert_eq!(
            DecoderResult::Corrupt(Token::RightParen),
            decode_chunk("[<(<(<(<{}))><([]([]()")
        );
    }

    #[test]
    fn find_syntax_error_score() {
        let input = parse_input(INPUT);
        assert_eq!(26397, score(input));
    }
}
