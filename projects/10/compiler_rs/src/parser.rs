use crate::lexer::Lexer;
use crate::token::{self, Token, TokenKind};

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

struct Parser {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    lex: Lexer,
    cur_line: String,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub(crate) fn new(file_path: String) -> Self {
        let out_path = file_path.split(".").next().unwrap().to_string() + ".xml";
        let in_f = File::open(&file_path).expect("Couldn't open file!");
        let out_f = File::create(&out_path).expect("Couldn't create file!");
        let reader = BufReader::new(in_f);
        let writer = BufWriter::new(out_f);
        let lex = Lexer::new(String::from(""));
        let mut p = Parser {
            reader,
            writer,
            lex,
            cur_line: String::from(""),
            cur_token: Token {
                Type: String::from("("),
                Literal: String::from("("),
            },
            peek_token: Token {
                Type: String::from("("),
                Literal: String::from("("),
            },
        };
        p.init();
        p
    }

    pub(crate) fn init(&mut self) {
        self.read_new_line();
        //self.next_token();
    }

    fn compile_class(&mut self) {
        // <class>
        self.write("<class>\n");
        //compiles a complete class.
        if self.peek_token_is(token::CLASS) {
            self.compile_term(); // keyword class
        }
        // identifier className
        if self.peek_token_is(token::IDENT) {
            self.compile_term();
        }
        // symbol '{'
        if self.peek_token_is(token::LBRACE) {
            self.compile_term();
        }

        while !self.peek_token_is(token::RBRACE) {
            println!("Peek token => {:?}", self.peek_token);
            if self.peek_token_is(token::FIELD) || self.peek_token_is(token::STATIC) {
                self.compile_class_vardec();
            } else {
                //self.compile_subroutine();
                self.next_token();
            }
        }
        // symbol '}'
        if self.peek_token_is(token::RBRACE) {
            self.compile_term();
        }
        // </class>
        self.write("</class>\n");
    }

    fn compile_class_vardec(&mut self) {
        //compiles a static declaration or a field declaration.
        // <classVarDec>
        self.write("<classVarDec>\n");
        //keyword (static| field)
        self.compile_term();
        //keyword type or identifier className
        self.compile_term();

        while !self.peek_token_is(token::SEMICOLON) {
            self.compile_term();
        }

        if self.peek_token_is(token::SEMICOLON) {
            self.compile_term();
        }

        // </classVarDec>
        self.write("</classVarDec>\n");
    }

    fn compile_subroutine(&mut self) {
        // compiles a complete method, function,
        // or constructor.
        //<subroutineDec>
        self.write("<subroutineDec>\n");

        //keyword (constructor| function| method)
        self.compile_term();
        // keyword or identifier (void| type)
        self.compile_term();
        // identifier subroutinename
        self.compile_term();
        // LPAREN
        self.compile_term();

        if self.cur_token_is(token::LPAREN) {
            self.compile_parameter_list();
        }

        // RPAREN
        self.compile_term();
        self.write("<subroutineBodyDec>\n");
        // LBRACE
        self.compile_term();

        // RBRACE
        self.compile_term();
        self.write("</subroutineBodyDec>\n");
        self.write("</subroutineDec>\n");
        //</subroutineDec>
    }
    fn compile_parameter_list(&mut self) {
        // compiles a (possibly empty) parameter
        //list, not including the enclosing “()”.
        self.write("<parameterList>\n");
        
        while !self.peek_token_is(token::RPAREN) {
            self.compile_term();
        }

        self.write("</parameterList>\n");
    }

    fn compile_vardec(&mut self) {}

    fn compile_statements(&mut self) {
        self.write("<statements>\n");




        self.write("</statements>\n");

    }

    fn compile_do(&mut self) {}

    fn compile_let(&mut self) {}

    fn compile_while(&mut self) {
        //compiles a sequence of statements, not
        // including the enclosing “{}”.
    }

    fn compile_return(&mut self) {
        // <returnStatement>
        // self.compile_term ''return keyword'
        // self.compile_expression
        // self.compile_term -> semicolon
        // </returnStatement>
    }

    fn compile_if(&mut self) {
        // <ifStatement>
        // compiles an if statement, possibly
        //with a trailing else clause.
        // </ifStatement>
    }

    fn compile_expression(&mut self) {}

    fn compile_term(&mut self) {
        self.next_token();
        //println!("{:?}", self.cur_token.Type);
        match self.cur_token.token_type() {
            TokenKind::Keyword(s) => {
                let s = format!("<keyword> {} </keyword>\n", &s);
                self.write(&s);
            }
            TokenKind::Symbol(s) => {
                let s = format!("<symbol> {} </symbol>\n", &s);
                self.write(&s);
            }
            TokenKind::Integer(s) => {
                let s = format!("<integerConstant> {} </integerConstant>\n", &s);
                self.write(&s);
            }
            TokenKind::StringC(s) => {
                let s = format!("<stringConstant> {} </stringConstant>\n", &s);
                self.write(&s);
            }
            TokenKind::Identifier(s) => {
                let s = format!("<identifier> {} </identifier>\n", &s);
                self.write(&s);
            }
        }
        //self.cur_token.toke_type.to_string()
    }

    fn compile_expression_list(&mut self) {
        // compiles a (possibly empty) comma-
        // separated list of expressions.
    }
    fn next_token(&mut self) {
        if let Some(t) = self.lex.next_token() {
            self.cur_token = self.peek_token.clone();
            self.peek_token = t;
        } else {
            self.read_new_line();
        }
    }

    fn cur_token_is(&self, typ: &str) -> bool {
        self.cur_token.Type == typ
    }

    fn peek_token_is(&self, typ: &str) -> bool {
        self.peek_token.Type == typ
    }

    fn read_new_line(&mut self) {
        let mut line = String::from("");
        let num_bytes_read = self
            .reader
            .read_line(&mut line)
            .expect("Error reading line!");
        if num_bytes_read == 0 {
            self.writer.flush().expect("Couldn't flush");
            return;
        }
        self.lex = Lexer::new(line);
        self.cur_line = self.lex.input.clone();
        println!("{:?}", self.cur_line);
        self.next_token();
    }

    fn write(&mut self, info: &str) {
        self.writer
            .write(info.as_bytes())
            .expect("Error writing buffer!");
    }

    pub(crate) fn parse(&mut self) {
        self.compile_class();
    }
}

#[test]
fn parser_test() {
    let mut pars = Parser::new("Square.jack".to_string());
    //println!("{:?} , {:?}", pars.cur_token, pars.peek_token);
    pars.parse();
}
