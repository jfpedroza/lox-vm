#[cfg(test)]
mod tests;

use crate::location::Loc;
use crate::utils::{is_alpha, is_alphanumeric, is_digit};

pub struct Scanner<'a> {
    start: std::str::Chars<'a>,
    current: std::str::Chars<'a>,
    length: usize,
    start_loc: Loc,
    current_loc: Loc,
}

#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub loc: Loc,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Question,
    Colon,
    Semicolon,

    // One or two character tokens
    Minus,
    MinusEqual,
    MinusMinus,
    Plus,
    PlusEqual,
    PlusPlus,
    Slash,
    SlashEqual,
    Star,
    StarEqual,
    Percent,
    PercentEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    Str,
    Integer,
    Float,

    // Keywords
    And,
    Break,
    Class,
    Continue,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, PartialEq, Fail)]
pub enum ScanningError {
    UnrecognizedCharacter(char, Loc),
    UnterminatedString(Loc),
    UnterminatedBlockComment(Loc),
}

type TokenRes<'a> = Result<Token<'a>, ScanningError>;

impl Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            start: source.chars(),
            current: source.chars(),
            length: 0,
            start_loc: Loc::new(0, 0),
            current_loc: Loc::new(0, 0),
        }
    }

    pub fn scan_token(&mut self) -> TokenRes<'a> {
        use TokenKind::*;
        self.skip_whitespace()?;
        self.start = self.current.clone();
        self.start_loc = self.current_loc;
        self.length = 0;

        let character = if let Some(ch) = self.advance() {
            ch
        } else {
            return Ok(self.make_token(EOF));
        };

        let token = match character {
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            '[' => self.make_token(LeftBracket),
            ']' => self.make_token(RightBracket),
            ',' => self.make_token(Comma),
            '.' => self.make_token(Dot),
            '?' => self.make_token(Question),
            ':' => self.make_token(Colon),
            ';' => self.make_token(Semicolon),
            '-' => {
                let kind = if self.matches('=') {
                    MinusEqual
                } else if self.matches('-') {
                    MinusMinus
                } else {
                    Minus
                };

                self.make_token(kind)
            }
            '+' => {
                let kind = if self.matches('=') {
                    PlusEqual
                } else if self.matches('+') {
                    PlusPlus
                } else {
                    Plus
                };

                self.make_token(kind)
            }
            '*' => {
                let kind = if self.matches('=') { StarEqual } else { Star };
                self.make_token(kind)
            }
            '/' => {
                let kind = if self.matches('=') { SlashEqual } else { Slash };
                self.make_token(kind)
            }
            '%' => {
                let kind = if self.matches('=') {
                    PercentEqual
                } else {
                    Percent
                };
                self.make_token(kind)
            }
            '!' => {
                let kind = if self.matches('=') { BangEqual } else { Bang };
                self.make_token(kind)
            }
            '=' => {
                let kind = if self.matches('=') { EqualEqual } else { Equal };
                self.make_token(kind)
            }
            '<' => {
                let kind = if self.matches('=') { LessEqual } else { Less };
                self.make_token(kind)
            }
            '>' => {
                let kind = if self.matches('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.make_token(kind)
            }
            '"' => self.recognize_string()?,
            ch if is_digit(ch) => self.recognize_number()?,
            ch if is_alpha(ch) => self.recognize_identifier()?,
            ch => return Err(self.unrecognized_character(ch)),
        };

        Ok(token)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current.next() {
            self.length += ch.len_utf8();
            self.current_loc.advance();
            Some(ch)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.current.clone().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.current.clone().nth(1)
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if predicate(ch) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_lexeme(&self) -> &'a str {
        &self.start.as_str()[..self.length]
    }

    fn make_token(&self, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            lexeme: self.get_lexeme(),
            loc: self.start_loc,
        }
    }

    fn recognize_string(&mut self) -> TokenRes<'a> {
        while let Some(character) = self.peek() {
            match character {
                '"' => break,
                '\\' => {
                    self.advance();
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.current_loc.new_line();
                }
                _ => {
                    self.advance();
                }
            }
        }

        if self.advance().is_none() {
            return Err(self.unterminated_string());
        }

        Ok(self.make_token(TokenKind::Str))
    }

    fn recognize_number(&mut self) -> TokenRes<'a> {
        self.advance_while(is_digit);

        let mut is_float = match (self.peek(), self.peek_next()) {
            (Some('.'), Some(ch)) if is_digit(ch) => {
                self.advance();
                self.advance_while(is_digit);
                true
            }
            _ => false,
        };

        if self.matches('e') || self.matches('E') {
            is_float = true;
            if !self.matches('+') {
                self.matches('-');
            };
            self.advance_while(is_digit);
        }

        let kind = if is_float {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };

        Ok(self.make_token(kind))
    }

    fn recognize_identifier(&mut self) -> TokenRes<'a> {
        self.advance_while(is_alphanumeric);
        let kind = self.identifier_kind();
        Ok(self.make_token(kind))
    }

    fn identifier_kind(&self) -> TokenKind {
        let ident = self.get_lexeme();
        let mut chars = ident.chars();
        let first = chars.next().unwrap();

        match first {
            'a' => Self::check_keyword(chars.as_str(), "nd", TokenKind::And),
            'b' => Self::check_keyword(chars.as_str(), "reak", TokenKind::Break),
            'c' => match chars.next() {
                Some('l') => Self::check_keyword(chars.as_str(), "ass", TokenKind::Class),
                Some('o') => Self::check_keyword(chars.as_str(), "ntinue", TokenKind::Continue),
                _ => TokenKind::Identifier,
            },
            'e' => Self::check_keyword(chars.as_str(), "lse", TokenKind::Else),
            'f' => match chars.next() {
                Some('a') => Self::check_keyword(chars.as_str(), "lse", TokenKind::False),
                Some('o') => Self::check_keyword(chars.as_str(), "r", TokenKind::For),
                Some('u') => Self::check_keyword(chars.as_str(), "n", TokenKind::Fun),
                _ => TokenKind::Identifier,
            },
            'i' => Self::check_keyword(chars.as_str(), "f", TokenKind::If),
            'n' => Self::check_keyword(chars.as_str(), "il", TokenKind::Nil),
            'o' => Self::check_keyword(chars.as_str(), "r", TokenKind::Or),
            'p' => Self::check_keyword(chars.as_str(), "rint", TokenKind::Print),
            'r' => Self::check_keyword(chars.as_str(), "eturn", TokenKind::Return),
            's' => Self::check_keyword(chars.as_str(), "uper", TokenKind::Super),
            't' => match chars.next() {
                Some('h') => Self::check_keyword(chars.as_str(), "is", TokenKind::This),
                Some('r') => Self::check_keyword(chars.as_str(), "ue", TokenKind::True),
                _ => TokenKind::Identifier,
            },
            'v' => Self::check_keyword(chars.as_str(), "ar", TokenKind::Var),
            'w' => Self::check_keyword(chars.as_str(), "hile", TokenKind::While),
            _ => TokenKind::Identifier,
        }
    }

    fn check_keyword(rest: &str, expected: &str, kind: TokenKind) -> TokenKind {
        if rest == expected {
            kind
        } else {
            TokenKind::Identifier
        }
    }

    fn skip_whitespace(&mut self) -> Result<(), ScanningError> {
        while let Some(character) = self.peek() {
            match character {
                '/' if self.peek_next() == Some('/') => {
                    self.advance_while(|ch| ch != '\n');
                }
                '/' if self.peek_next() == Some('*') => {
                    self.skip_block_comment()?;
                }
                ch if !ch.is_ascii_whitespace() => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }

            if character == '\n' {
                self.current_loc.new_line();
            }
        }

        Ok(())
    }

    fn skip_block_comment(&mut self) -> Result<(), ScanningError> {
        let mut depth = 0usize;
        while let Some(ch) = self.advance() {
            match (ch, self.peek()) {
                ('/', Some('*')) => {
                    self.advance();
                    depth += 1;
                }
                ('*', Some('/')) => {
                    self.advance();
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }
                }
                ('\n', _) => self.current_loc.new_line(),
                _ => (),
            }
        }

        if depth == 0 {
            Ok(())
        } else {
            Err(self.unterminated_block_comment())
        }
    }

    fn unrecognized_character(&self, character: char) -> ScanningError {
        ScanningError::UnrecognizedCharacter(character, self.start_loc)
    }

    fn unterminated_string(&self) -> ScanningError {
        ScanningError::UnterminatedString(self.current_loc)
    }

    fn unterminated_block_comment(&self) -> ScanningError {
        ScanningError::UnterminatedBlockComment(self.current_loc)
    }
}
