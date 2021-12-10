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
    /// Expected token char and found char
    Corrupt(char, char),
    /// Remaining stack as string
    Incomplete(String),
}

fn corrupted_score(lines: &[String]) -> u32 {
    let score_table: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();

    let mut score = 0;
    for line in lines {
        if let DecoderResult::Corrupt(_expected, found) = decode_chunk(&line) {
            score += score_table[&found];
        }
    }
    score
}

fn incomplete_score(lines: &[String]) -> u32 {
    let score_table: HashMap<char, u32> = [(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect();

    let mut scores: Vec<u32> = Vec::new();
    for line in lines {
        if let DecoderResult::Incomplete(tokens) = decode_chunk(&line) {
            let score = tokens.chars().fold(0_u32, |product, token| {
                product * 5 + score_table[&token]
            });
            scores.push(score);
        }
    }

    // Sort all scores
    scores.sort();

    // Find the middle score
    let index = scores.len() / 2;

    scores[index]
}

/// Decodes the chunk and returns true if pairs match fully
fn decode_chunk(chunk: &str) -> DecoderResult {
    fn missing_tokens(list: &[Token]) -> String {
        list.iter().rev().map(|t| t.opposite().as_char()).join("")
    }

    let mut stack = Vec::new();

    for token in chunk.chars().map(Token::from).collect_vec() {
        if token.opens() {
            stack.push(token);
        } else if let Some(last_token) = stack.pop() {
            if !last_token.matches(&token) {
                return DecoderResult::Corrupt(last_token.opposite().as_char(), token.as_char());
            }
        }
    }

    if stack.is_empty() {
        DecoderResult::Ok
    } else {
        DecoderResult::Incomplete(missing_tokens(&stack))
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

    let total = corrupted_score(&chunks);
    dbg!(total);

    let total = incomplete_score(&chunks);
    dbg!(total);
}

#[cfg(test)]
mod tests {
    use crate::{DecoderResult, corrupted_score, decode_chunk, incomplete_score, parse_input};

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
    fn test_short_corrupted_lines() {
        assert_eq!(DecoderResult::Corrupt(')', ']'), decode_chunk("(]"));
        assert_eq!(DecoderResult::Corrupt('}', '>'), decode_chunk("{()()()>"));
        assert_eq!(DecoderResult::Corrupt(')', '}'), decode_chunk("(((()))}"));
        assert_eq!(DecoderResult::Corrupt('>', ')'), decode_chunk("<([]){()}[{}])"));
    }

    #[test]
    fn test_corrupted_lines() {
        assert_eq!(
            DecoderResult::Corrupt(']', '}'),
            decode_chunk("{([(<{}[<>[]}>{[]{[(<()>")
        );
        assert_eq!(
            DecoderResult::Corrupt(']', ')'),
            decode_chunk("[[<[([]))<([[{}[[()]]]")
        );
        assert_eq!(
            DecoderResult::Corrupt(')', ']'),
            decode_chunk("[{[{({}]{}}([{[{{{}}([]")
        );
        assert_eq!(
            DecoderResult::Corrupt('>', ')'),
            decode_chunk("[<(<(<(<{}))><([]([]()")
        );
        assert_eq!(
            DecoderResult::Corrupt(']', '>'),
            decode_chunk("<{([([[(<>()){}]>(<<{{")
        );
    }

    #[test]
    fn test_incomplete_lines() {
        assert_eq!(
            DecoderResult::Incomplete("}}]])})]".to_string()),
            decode_chunk("[({(<(())[]>[[{[]{<()<>>"),
        );
        assert_eq!(
            DecoderResult::Incomplete(")}>]})".to_string()),
            decode_chunk("[(()[<>])]({[<{<<[]>>("),
        );
        assert_eq!(
            DecoderResult::Incomplete("}}>}>))))".to_string()),
            decode_chunk("(((({<>}<{<{<>}{[]{[]{}"),
        );
        assert_eq!(
            DecoderResult::Incomplete("]]}}]}]}>".to_string()),
            decode_chunk("{<[[]]>}<{[{[{[]{()[[[]"),
        );
    }

    #[test]
    fn find_corrupt_score() {
        let input = parse_input(INPUT);
        assert_eq!(26397, corrupted_score(&input));
    }

    #[test]
    fn find_incomlete_score() {
        let input = parse_input(INPUT);
        assert_eq!(288957, incomplete_score(&input));
    }
}
