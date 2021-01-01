use crate::lexer::Lexer;
use crate::token::Token;

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
        Parser {
            reader,
            writer,
            lex,
            cur_line: String::from(""),
            cur_token: Token {
                Type: String::from(""),
                Literal: String::from(""),
            },
            peek_token: Token {
                Type: String::from(""),
                Literal: String::from(""),
            },
        }
    }
    fn compile_class(&mut self) {
        // <class>
        //compiles a complete class.
        //self.next_token()
        //self.compile_term();    keyword class
        //self.next_token()
        //self.compile_term();   identifier className
        //self.next_token()
        //self.compile_term();    symbol '{'

        // while self.next_token != toke.EOF {
        // if self.peek_token == static | field => self.compile_class_vardec
        // else  self.subroutine

        //self.compile_term();    symbol '}'
        // </class>
    }
    //self.compile_term();    symol '}'
    fn compile_class_vardec(&mut self) {
        //compiles a static declaration or a field declaration.
        // self.compile_term();    keyword (static| field)
        // self.compile_term();     keyword type or identifier className
        //while cur_token != token.SEMICOLON {self.next_token(); self.compile_term()}
        // self.compile_term() -> token SEMICOLON
    }
    fn compile_subroutine(&mut self) {
        //<subroutineDec>
        // compiles a complete method, function,
        // or constructor.
        //</subroutineDec>
    }
    fn compile_parameter_list(&mut self) {
        // compiles a (possibly empty) parameter
        //list, not including the enclosing “()”.
    }

    fn compile_vardec(&mut self) {}

    fn compile_statements(&mut self) {}

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
        // self.next_token()
        //self.cur_token.toke_type.to_string()
    }

    fn compile_expression_list(&mut self) {
        // compiles a (possibly empty) comma-
        // separated list of expressions.
    }
    fn next_token(&mut self) {
        // todo: check self.lex.has_more_tokens() == true
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token().unwrap();
    }

    fn cur_token_is(&self, typ: &str) -> bool {
        self.cur_token.Type == typ
    }

    fn peek_token_is(&self, typ: &str) -> bool {
        self.peek_token.Type == typ
    }

    // fn read_new_line(&mut self) {
    //     self.lex = lex
    // }
}
