use token::{Token, TokenKind};

use crate::token;
#[derive(Debug)]
pub(crate) struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    pub(crate) fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: b'0',
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.advance();
        self.skip_whitespace();
        match self.ch {
            b'=' => Token::new(token::EQ, std::slice::from_ref(&self.ch)),
            b'+' => Token::new(token::PLUS, std::slice::from_ref(&self.ch)),
            b'-' => Token::new(token::MINUS, std::slice::from_ref(&self.ch)),
            b'*' => Token::new(token::ASTERISK, std::slice::from_ref(&self.ch)),
            b'<' => Token::new(token::LT, std::slice::from_ref(&self.ch)),
            b'>' => Token::new(token::GT, std::slice::from_ref(&self.ch)),
            b'.' => Token::new(token::DOT, std::slice::from_ref(&self.ch)),
            b';' => Token::new(token::SEMICOLON, std::slice::from_ref(&self.ch)),
            b',' => Token::new(token::COMMA, std::slice::from_ref(&self.ch)),
            b'{' => Token::new(token::LBRACE, std::slice::from_ref(&self.ch)),
            b'}' => Token::new(token::RBRACE, std::slice::from_ref(&self.ch)),
            b'(' => Token::new(token::LPAREN, std::slice::from_ref(&self.ch)),
            b')' => Token::new(token::RPAREN, std::slice::from_ref(&self.ch)),
            b'[' => Token::new(token::LBRACKET, std::slice::from_ref(&self.ch)),
            b']' => Token::new(token::RBRACKET, std::slice::from_ref(&self.ch)),
            b'&' => Token::new(token::AND, std::slice::from_ref(&self.ch)),
            b'|' => Token::new(token::OR, std::slice::from_ref(&self.ch)),
            b'~' => Token::new(token::NOT, std::slice::from_ref(&self.ch)),
            b'/' => Token::new(token::SLASH, std::slice::from_ref(&self.ch)),
            b'"' => Token {
                Type: token::STRING_CONST,
                Literal: self.read_string(),
            },
            b'0' => Token {
                Type: token::EOF,
                Literal: "",
            },
            _ => {
                if is_letter(self.ch) {
                    let re = self.read_identifier();
                    Token {
                        Type: token::lookup_ident(re),
                        Literal: re,
                    }
                } else if is_digit(self.ch) {
                    Token {
                        Type: token::INT_CONST,
                        Literal: self.read_number(),
                    }
                } else {
                    Token::new(token::ILLEGAL, std::slice::from_ref(&self.ch))
                }
            }
        }
    }

    fn has_more_tokens(&self) -> bool {
        self.read_position < self.input.len()
    }

    fn advance(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            b'0'
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.advance()
        }
    }

    // fn skip_comments(&mut self) -> Token {
    //     // three type of comments

    //     // in line comment
    //     if self.peek_char() == b'/' {
    //         while self.ch  != b'\n' {
    //             self.advance();
    //         }
    //     }
    //     // block comment and api
    //     if self.peek_char() == b'*' {
    //         self.advance(); // cur = first *
    //         if self.peek_char() == b'*' {
    //             self.advance(); // cur second *
    //         }
    //         self.advance(); // overcome *
    //         while self.ch != b'*' {
    //             self.advance();
    //         }
    //         self.advance() // got to / token
    //     }

    //     self.next_token()

    // }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.advance()
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.position]).unwrap()
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.advance()
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.position]).unwrap()
    }

    fn read_string(&mut self) -> &str {
        println!("{:?}", self.ch);
        self.advance();
        let position = self.position;
        while self.ch != b'"' {
            self.advance()
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.position]).unwrap()
    }

    pub(crate) fn tokenize(&mut self) {
        while self.has_more_tokens() {
            let token = self.next_token();
            let out = match token.token_type() {
                TokenKind::Keyword(s) => format!("<keyword> {} </keyword>", s),
                TokenKind::Symbol(s) => {
                    if token.Type == token::GT {
                        format!("<symbol> {} </symbol>", "&gt;")
                    } else if token.Type == token::LT {
                        format!("<symbol> {} </symbol>", "&lt;")
                    } else if token.Type == token::AND {
                        format!("<symbol> {} </symbol>", "&amp;")
                    } else {
                        format!("<symbol> {} </symbol>", s)
                    }
                }
                TokenKind::Integer(s) => format!("<integerConstant> {} </integerConstant>", s),
                TokenKind::StringC(s) => format!("<stringConstant> {} </stringConstant>", s),
                TokenKind::Identifier(s) => format!("<identifier> {} </identifier>", s),
            };
            println!("{}", out);
        }
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

