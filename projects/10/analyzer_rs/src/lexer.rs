use token::{Token, TokenKind};

use crate::token;
#[derive(Debug)]
pub(crate) struct Lexer {
    pub input: String,
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
            ch: b'\0',
        }
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        self.advance();
        self.skip_whitespace();
        match self.ch {
            b'=' => Some(Token::new(
                String::from(token::EQ),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'+' => Some(Token::new(
                String::from(token::PLUS),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'-' => Some(Token::new(
                String::from(token::MINUS),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'*' => {
                if self.position == 1 {
                    self.skip_comments()
                } else {
                    Some(Token::new(
                        String::from(token::ASTERISK),
                        std::slice::from_ref(&self.ch).to_vec(),
                    ))
                }
            }
            b'<' => Some(Token::new(
                String::from(token::LT),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'>' => Some(Token::new(
                String::from(token::GT),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'.' => Some(Token::new(
                String::from(token::DOT),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b';' => Some(Token::new(
                String::from(token::SEMICOLON),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b',' => Some(Token::new(
                String::from(token::COMMA),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'{' => Some(Token::new(
                String::from(token::LBRACE),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'}' => Some(Token::new(
                String::from(token::RBRACE),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'(' => Some(Token::new(
                String::from(token::LPAREN),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b')' => Some(Token::new(
                String::from(token::RPAREN),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'[' => Some(Token::new(
                String::from(token::LBRACKET),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b']' => Some(Token::new(
                String::from(token::RBRACKET),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'&' => Some(Token::new(
                String::from(token::AND),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'|' => Some(Token::new(
                String::from(token::OR),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'~' => Some(Token::new(
                String::from(token::NOT),
                std::slice::from_ref(&self.ch).to_vec(),
            )),
            b'/' => {
                let n_char = self.peek_char();
                if n_char == b'/' || n_char == b'*' {
                    self.skip_comments()
                } else {
                    Some(Token::new(
                        String::from(token::SLASH),
                        std::slice::from_ref(&self.ch).to_vec(),
                    ))
                }
            }

            b'"' => Some(Token {
                Type: String::from(token::STRING_CONST),
                Literal: self.read_string().to_string(),
            }),
            b'\0' => None,
            _ => {
                if is_letter(self.ch) {
                    let re = self.read_identifier();
                    Some(Token {
                        Type: token::lookup_ident(re).to_string(),
                        Literal: re.to_string(),
                    })
                } else if is_digit(self.ch) {
                    Some(Token {
                        Type: String::from(token::INT_CONST),
                        Literal: self.read_number().to_string(),
                    })
                } else {
                    Some(Token::new(
                        String::from(token::ILLEGAL),
                        std::slice::from_ref(&self.ch).to_vec(),
                    ))
                }
            }
        }
    }

    pub(crate) fn has_more_tokens(&self) -> bool {
        self.read_position < self.input.len()
    }

    fn advance(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            b'\0'
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.advance()
        }
    }

    fn skip_comments(&mut self) -> Option<Token> {
        // in line comment
        if self.peek_char() == b'/' && self.ch == b'/' {
            while self.ch != b'\n' && self.ch != b'\0' {
                //println!("there {:?}", self.ch);
                self.advance();
            }
            return None;
        } else if self.ch == b'*' {
            self.advance();
            while self.ch != b'*' || self.peek_char() != b'/' {
                if self.ch == b'\0' {
                    return None;
                }
                self.advance();
            }
        } else {
            self.advance(); // cur = first *
            if self.peek_char() == b'*' {
                self.advance(); // cur second *
            }
            self.advance(); // overcome *
            while self.ch != b'*' || self.peek_char() != b'/' {
                if self.ch == b'\0' {
                    return None;
                }
                self.advance();
            }
            self.advance(); // go to / token
        }
        self.next_token()
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.peek_char()) {
            self.advance()
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.read_position]).unwrap()
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.peek_char()) {
            self.advance()
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.read_position]).unwrap()
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
            if let Some(t) = self.next_token() {
                if t.Type != token::EOF {
                    println!("{}", Lexer::token_to_xml(&t));
                }
            }
        }
    }

    pub(crate) fn token_to_xml(token: &Token) -> String {
        match token.token_type() {
            TokenKind::Keyword(s) => format!("<keyword> {} </keyword>\n", s),
            TokenKind::Symbol(s) => {
                if token.Type == token::GT {
                    format!("<symbol> {} </symbol>\n", "&gt;")
                } else if token.Type == token::LT {
                    format!("<symbol> {} </symbol>\n", "&lt;")
                } else if token.Type == token::AND {
                    format!("<symbol> {} </symbol>\n", "&amp;")
                } else {
                    format!("<symbol> {} </symbol>\n", s)
                }
            }
            TokenKind::Integer(s) => format!("<integerConstant> {} </integerConstant>\n", s),
            TokenKind::StringC(s) => format!("<stringConstant> {} </stringConstant>\n", s),
            TokenKind::Identifier(s) => format!("<identifier> {} </identifier>\n", s),
        }
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}
