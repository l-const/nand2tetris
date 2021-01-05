use std::fmt::{self, Display};

pub type TokenType = &'static str;

pub(crate) enum TokenKind {
    Keyword(String),
    Symbol(String),
    Integer(String),
    StringC(String),
    Identifier(String),
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Keyword(s) => write!(f, "<keyword> {} </keyword>\n", s),
            TokenKind::Symbol(s) => write!(f, "<symbol> {} </symbol>\n", s),
            TokenKind::Integer(s) => write!(f, "<integerConstant> {} </integerConstant>\n", s),
            TokenKind::StringC(s) => write!(f, "<stringConstant> {} </stringConstant>\n", s),
            TokenKind::Identifier(s) => write!(f, "<identifier> {} </identifier>\n", s),
        }
    }
}

pub(crate) const ILLEGAL: &'static str = "ILLEGAL";
pub(crate) const EOF: &'static str = "EOF";

// Identifiers and Literals
pub(crate) const IDENT: &'static str = "IDENT"; // add, foobar, x, y
pub(crate) const INT_CONST: &'static str = "INT"; // 1343456
pub(crate) const STRING_CONST: &'static str = "STRING"; // "DDSDSDS"

// Symbols
pub(crate) const LPAREN: &'static str = "(";
pub(crate) const RPAREN: &'static str = ")";
pub(crate) const LBRACE: &'static str = "{";
pub(crate) const RBRACE: &'static str = "}";
pub(crate) const LBRACKET: &'static str = "[";
pub(crate) const RBRACKET: &'static str = "]";
pub(crate) const DOT: &'static str = ".";
pub(crate) const COMMA: &'static str = ",";
pub(crate) const SEMICOLON: &'static str = ";";

// Operators
// Arithmetic
pub(crate) const PLUS: &'static str = "+";
pub(crate) const MINUS: &'static str = "-";
pub(crate) const ASTERISK: &'static str = "*";
pub(crate) const SLASH: &'static str = "/";
// Logical
pub(crate) const AND: &'static str = "&";
pub(crate) const OR: &'static str = "|";
// Comparison
pub(crate) const LT: &'static str = ">";
pub(crate) const GT: &'static str = "<";
pub(crate) const EQ: &'static str = "=";
pub(crate) const NOT: &'static str = "~";

// Keywords
pub(crate) const CLASS: TokenType = "CLASS";
pub(crate) const CONSTRUCTOR: TokenType = "CONSTRUCTOR";
pub(crate) const FUNCTION: TokenType = "FUNCTION";
pub(crate) const METHOD: TokenType = "METHOD";
pub(crate) const FIELD: TokenType = "FIELD";
pub(crate) const STATIC: TokenType = "STATIC";
pub(crate) const VAR: TokenType = "VAR";
pub(crate) const INT_K: TokenType = "INT_K";
pub(crate) const CHAR_K: TokenType = "CHAR_K";
pub(crate) const BOOLEAN_K: TokenType = "BOOLEAN_K";
pub(crate) const VOID: TokenType = "VOID";
pub(crate) const TRUE: TokenType = "TRUE";
pub(crate) const FALSE: TokenType = "FALSE";
pub(crate) const NULL: TokenType = "NULL";
pub(crate) const THIS: TokenType = "THIS";
pub(crate) const LET: TokenType = "LET";
pub(crate) const DO: TokenType = "DO";
pub(crate) const IF: TokenType = "IF";
pub(crate) const ELSE: TokenType = "ELSE";
pub(crate) const WHILE: TokenType = "WHILE";
pub(crate) const RETURN: TokenType = "RETURN";

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) Type: String,
    pub(crate) Literal: String,
}

impl Token {
    pub fn new(token_type: String, ch: Vec<u8>) -> Self {
        Token {
            Type: token_type,
            Literal: std::str::from_utf8(&ch).unwrap().to_string(),
        }
    }

    pub fn string_val(&self) -> Option<String> {
        match self.token_type() {
            TokenKind::StringC(s) => Some(s),
            _ => None,
        }
    }

    pub fn token_type(&self) -> TokenKind {
        match self.Type.as_ref() {
            INT_CONST => TokenKind::Integer(String::from(&self.Literal)),
            STRING_CONST => TokenKind::StringC(String::from(&self.Literal)),
            IDENT => TokenKind::Identifier(String::from(&self.Literal)),
            LPAREN | RPAREN | LBRACE | RBRACE | LBRACKET | RBRACKET | DOT | COMMA | SEMICOLON
            | PLUS | MINUS | ASTERISK | SLASH | AND | OR | LT | GT | EQ | NOT => {
                TokenKind::Symbol(String::from(&self.Literal))
            }
            _ => TokenKind::Keyword(String::from(&self.Literal)),
        }
    }

    pub fn symbol(&self) -> Option<String> {
        match self.token_type() {
            TokenKind::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn keyword(&self) -> Option<String> {
        match self.token_type() {
            TokenKind::Keyword(s) => Some(s),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<String> {
        match self.token_type() {
            TokenKind::Identifier(s) => Some(s),
            _ => None,
        }
    }

    pub fn integer_val(&self) -> Option<String> {
        match self.token_type() {
            TokenKind::Integer(s) => Some(s),
            _ => None,
        }
    }
}

pub(crate) fn lookup_ident(ident: &str) -> String {
    let result = match ident {
        "class" => CLASS,
        "constructor" => CONSTRUCTOR,
        "function" => FUNCTION,
        "method" => METHOD,
        "field" => FIELD,
        "static" => STATIC,
        "var" => VAR,
        "int" => INT_K,
        "char" => CHAR_K,
        "boolean" => BOOLEAN_K,
        "void" => VOID,
        "true" => TRUE,
        "false" => FALSE,
        "null" => NULL,
        "this" => THIS,
        "let" => LET,
        "do" => DO,
        "if" => IF,
        "else" => ELSE,
        "while" => WHILE,
        "return" => RETURN,
        _ => IDENT,
    };
    String::from(result)
}
